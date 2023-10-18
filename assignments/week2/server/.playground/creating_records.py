#!/usr/bin/env python
import load as _

from pathlib import Path
from os import getenv
import socket

ADDR = getenv('VENNBASE_ADDR')
PORT = int(getenv('VENNBASE_PORT'))

conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
conn.connect((ADDR, PORT))

def save_record_to_vennbase(path: Path):
    with open(path, 'rb') as r:
        print("sending as binary...")
        # this file is gb long, so is better to send it as chunks
        conn.send(b'save image/jpeg\n')
        conn.send(r.read())

save_record_to_vennbase('./duck.jpeg')

# while True:
#     query = input('>>> ') + '\n'
#     print(query.encode())
#     conn.send(query.encode())

#     data = conn.recv(1024)
#     print(data)

