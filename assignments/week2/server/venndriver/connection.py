import base64
import socket
from pathlib import Path
from os import getenv

ADDR = getenv('VENNBASE_ADDR', '127.0.0.1')
PORT = int(getenv('VENNBASE_PORT', 1834))

# TODO: send Content-Length
def save_record_to_vennbase(base64_record: str, mimetype: str):
    conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    conn.connect((ADDR, PORT))

    # decode base64 to binary in order to save it as a file
    data = base64.b64decode(base64_record)

    # this file is gb long, so is better to send it as chunks
    conn.send(f'save {mimetype}\n'.encode())
    conn.send(data)
    # send EOF but still read the response
    conn.shutdown(socket.SHUT_WR)
    uuid = conn.recv(1024).decode()
    return uuid
