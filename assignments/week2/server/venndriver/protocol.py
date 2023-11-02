import socket, base64
from uuid import UUID
from os import getenv
from pathlib import Path

ADDR = getenv('VENNBASE_ADDR', '127.0.0.1')
PORT = int(getenv('VENNBASE_PORT', 1834))

def recv_until(conn: socket.socket, delimiter: bytes) -> bytes:
    data = b''
    while True:
        if (b := conn.recv(1)) == delimiter:
            break
        data += b
    return data

def get_first_line(conn: socket.socket) -> bytes:
    return recv_until(conn, b'\n')

# TODO: send Content-Length
def save_record_to_vennbase(path: Path, mimetype: str, tags: list[str] = []):
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
    conn.settimeout(30)

    if not path.exists():
        print(path)
        raise Exception('File does not exist')

    with open(path, 'rb') as r:
        print(f'sending {mimetype} as binary...')
        # this file is gb long, so is better to send it as chunks
        conn.sendall(f'save {mimetype} {len(tags)}'.encode())
        for tag in tags:
            conn.sendall(f'\n{tag}'.encode())
        conn.sendall(b'\n' + r.read())
        # send EOF but still read the response
        conn.shutdown(socket.SHUT_WR)
        status, uuid = get_first_line(conn).split(b' ')
        if status == 'ERROR':
            raise Exception('Vennbase error')
        return uuid

def save_record_to_vennbase(base64_record: str, mimetype: str, tags: list[str]) -> UUID:
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
    conn.settimeout(30)

    # decode base64 to binary in order to save it as a file
    data = base64.b64decode(base64_record)
    print(f'parsed with len={len(data)}')

    # this file is gb long, so is better to send it as chunks
    print(f"saving as {mimetype}")
    conn.sendall(f'save {mimetype} {len(tags)}\n'.encode())

    for tag in tags:
        conn.sendall(f'{tag}\n'.encode())

    conn.sendall(data)
    # send EOF but still read the response
    conn.shutdown(socket.SHUT_WR)
    try:
        uuid = conn.recv(1024).decode()
    except socket.timeout:
        raise ConnectionError('Connection timed out')
    conn.close()
    return UUID(uuid)

def get_record_by_id(id: UUID, resize: str | None = None) -> tuple[bytes, str]:
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
    if resize:
        conn.sendall(f'get {resize} {id}\n'.encode())
    else:
        conn.sendall(f'get {id}\n'.encode())
    conn.settimeout(30)

    # If the uuid exists, a header is returned in the following format:
    #  <mimetype> <length>\n
    # Otherwise, this header is returned:
    #  NOT_FOUND 0\n
    header = get_first_line(conn)

    mimetype, record_length = header.split(b' ')
    if mimetype == 'NOT_FOUND':
        raise FileNotFoundError(f'Vennbase id {id} not found')

    try:
        mimetype = mimetype.decode('ascii')
        record_length = int(record_length.decode('ascii'))
    except ValueError:
        raise ValueError('Invalid response from server')

    data = b''
    while len(data) != record_length:
        try:
            data += conn.recv(record_length)
        except socket.timeout:
            raise ConnectionError('Connection timed out')

    conn.shutdown(socket.SHUT_RDWR)
    conn.close()

    return (data, mimetype)

class QueriedRecordInformation:
    def __init__(self, uuid: UUID, mimetype: str, tags: list[str]) -> None:
        self.uuid = uuid
        self.mimetype = mimetype
        self.tags = tags

    def __str__(self) -> str:
        return f'{self.uuid} {self.mimetype} {self.tags}'

def query_vennbase(query: str, skip: int=0, limit: int=100) -> list[QueriedRecordInformation]:
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
    conn.settimeout(30)

    conn.sendall(f'query {query}\n'.encode())
    # conn.sendall(f'query {query} skip={skip} limit={limit}\n'.encode())
    # send EOF but still read the response
    conn.shutdown(socket.SHUT_WR)
    header = get_first_line(conn)

    status, count = header.split(b' ')

    try:
        status = status.decode('ascii')
        count = int(count.decode('ascii'))
    except ValueError:
        raise ValueError('Invalid response from server')

    if status == 'ERROR':
        raise ValueError()

    if status == 'OK' and count == '0':
        return []

    results = ''
    try:
        while True:
            current = conn.recv(1024).decode()
            if current == '':
                break
            results += current
    except socket.timeout:
        raise ConnectionError('Connection timed out')

    line = 0
    result_lines = results.split('\n')

    queried_records: list[QueriedRecordInformation] = []
    for _ in range(count):
        uuid = UUID(result_lines[line])
        mimetype = result_lines[line + 1]
        tags_number = int(result_lines[line + 2])
        tags: list[str] = []
        for t in range(tags_number):
            tags.append(result_lines[line + 3 + t])
        line += 3 + tags_number
        queried_records.append(
            QueriedRecordInformation(uuid, mimetype, tags)
        )
    conn.close()
    return queried_records
