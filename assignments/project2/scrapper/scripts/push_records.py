#!/usr/bin/env python
from pathlib import Path
import requests
import base64
import socket
import json
import os

ADDR = os.getenv('VENNBASE_ADDR', '127.0.0.1')
PORT = int(os.getenv('VENNBASE_PORT', 1834))
OUTPUT_DIR = 'scrapped-data'
JSON_FILE = 'records.json'

def save_record_to_vennbase(path: Path, mimetype: str, tags: list[str] = []):
    """
    Saves the records directly to Vennbse without passing through the API
    """
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))
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
        uuid = conn.recv(1024).decode()
    return uuid

def post_record(path: Path, filename: str, mimetype: str, tags: list[str]):
    """
    Calls the API to save the record
    """
    with open(path, 'rb') as f:
        print(filename, mimetype)
        data = f.read()
        print(f'len={len(data)}')
        response = requests.post(
            'http://127.0.0.1:8000/api/records/',
            json={
                'name': filename,
                'tags': list(map(lambda t: { 'name': t }, tags)),
                'mimetype': mimetype,
                'base64_record': base64.b64encode(data).decode()
            }
        )
        print(response, '\n')

with open(Path(OUTPUT_DIR, JSON_FILE)) as f:
    records = json.loads(f.read())
    for record in records[:]:
        filename: str = record['filename']
        mimetype: str = record['mimetype']
        tags: list[str] = record['tags']
        path = Path(OUTPUT_DIR, filename)
        post_record(Path(path), filename, mimetype, tags)
