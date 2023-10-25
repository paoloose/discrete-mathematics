import base64
import socket
from uuid import UUID
from os import getenv

ADDR = getenv('VENNBASE_ADDR', '127.0.0.1')
PORT = int(getenv('VENNBASE_PORT', 1834))

# TODO: send Content-Length
def save_record_to_vennbase(base64_record: str, mimetype: str) -> UUID:
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
    conn.settimeout(3)

    # decode base64 to binary in order to save it as a file
    data = base64.b64decode(base64_record)
    print(f'parsed with len={len(data)}')

    # this file is gb long, so is better to send it as chunks
    print(f"saving as {mimetype}")
    conn.sendall(f'save {mimetype}\n'.encode())
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
    header = b''
    while True:
        if (b := conn.recv(1)) == b'\n':
            break
        header += b

    mimetype, record_length = header.split(b' ')
    if mimetype == 'NOT_FOUND':
        raise FileNotFoundError(f'Vennbase id {id} not found')

    try:
        mimetype = mimetype.decode('ascii')
        record_length = int(record_length.decode('ascii'))
    except ValueError:
        pass
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

def query_vennbase(query: str, skip: int, limit: int):
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
    conn.settimeout(3)

    conn.sendall(f'query {query} skip={skip} limit={limit}\n'.encode())
    # send EOF but still read the response
    conn.shutdown(socket.SHUT_WR)
    try:
        uuid = conn.recv(1024).decode()
    except socket.timeout:
        raise ConnectionError('Connection timed out')
    conn.close()
    return UUID(uuid)
