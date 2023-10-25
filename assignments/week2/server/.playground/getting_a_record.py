#!/usr/bin/env python
import load as _

from pathlib import Path
from uuid import UUID
from os import getenv
from venndriver.protocol import query_vennbase
import socket

ADDR = getenv('VENNBASE_ADDR', '127.0.0.1')
PORT = int(getenv('VENNBASE_PORT', 1834))

def get_record_by_id(id: UUID, save_path: Path):
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
    conn.sendall(f'get {id}\n'.encode())
    conn.settimeout(3)

    header = b''
    while True:
        if (b := conn.recv(1)) == b'\n':
            break
        header += b
    print('header:', header)

    mimetype, record_length = header.split(b' ')
    try:
        mimetype = mimetype.decode('ascii')
        record_length = int(record_length.decode('ascii'))
    except ValueError:
        pass
        raise ValueError('Invalid response from server')

    data = conn.recv(record_length)
    print('read: ', len(data))

    with open(save_path, 'wb') as f:
        f.write(data)

    conn.shutdown(socket.SHUT_RDWR)
    conn.close()

# get_record_by_id(
#     UUID('bad4130b-e744-914f-ac14-33dbad500a59'),
#     Path('./test2.png')
# )
