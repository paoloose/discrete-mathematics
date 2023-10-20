#!/usr/bin/env python
from time import sleep
import load as _

from pathlib import Path
import os
import requests
import base64
import socket

ADDR = os.getenv('VENNBASE_ADDR', '127.0.0.1')
PORT = int(os.getenv('VENNBASE_PORT', 1834))

def save_record_to_vennbase(path: Path, mimetype: str):
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
    if not path.exists():
        raise Exception('File does not exist')

    with open(path, 'rb') as r:
        print(f'sending {mimetype} as binary...')
        # this file is gb long, so is better to send it as chunks
        conn.sendall(f'save {mimetype}\n'.encode())
        conn.sendall(r.read())
        # send EOF but still read the response
        conn.shutdown(socket.SHUT_WR)
        uuid = conn.recv(1024).decode()
    return uuid

def post_record(path: Path, filename: str, mimetype: str):
    with open(path, 'rb') as f:
        print(filename, mimetype)
        data = f.read()
        print(f'len={len(data)}\n')
        response = requests.post(
            'http://127.0.0.1:8000/api/records/',
            json={
                'name': filename,
                'tags': [],
                'mimetype': mimetype,
                'base64_record': base64.b64encode(data).decode()
            }
        )
        print(response)

from mimetypes import guess_type

for dir, _, files in os.walk('/home/paolo/.assets/wallpapers'):
    for filename in files:
        path = Path(dir, filename)
        mimetype, _ = guess_type(path)
        if not mimetype:
            print('skipping', path)
            continue
        post_record(path, filename, mimetype)
