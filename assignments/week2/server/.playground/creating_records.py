#!/usr/bin/env python
from time import sleep
import load as _

from pathlib import Path
from os import getenv
import socket

ADDR = getenv('VENNBASE_ADDR', '127.0.0.1')
PORT = int(getenv('VENNBASE_PORT', 1834))

def save_record_to_vennbase(path: Path, mimetype: str):
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
    if not path.exists():
        raise Exception("File does not exist")

    with open(path, 'rb') as r:
        print("sending as binary...")
        # this file is gb long, so is better to send it as chunks
        conn.send(f'save {mimetype}\n'.encode())
        conn.send(r.read())
        # send EOF but still read the response
        conn.shutdown(socket.SHUT_WR)
        uuid = conn.recv(1024).decode()
    return uuid

save_record_to_vennbase(Path('./profile.png'), 'image/png')

# while True:
#     query = input('>>> ') + '\n'
#     print(query.encode())
#     conn.send(query.encode())

#     data = conn.recv(1024)
#     print(data)

