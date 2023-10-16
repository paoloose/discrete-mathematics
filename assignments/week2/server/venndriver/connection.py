from os import getenv
from dotenv import load_dotenv
import socket

if not load_dotenv():
    raise Exception("Failed to load .env file")

conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

ADDR = getenv('VENNBASE_ADDR')
PORT = int(getenv('VENNBASE_PORT'))
conn.connect((ADDR, PORT))

while True:
    query = input('>>> ') + '\n'
    print(query.encode())
    conn.send(query.encode())

    data = conn.recv(1024)
    print(data)
