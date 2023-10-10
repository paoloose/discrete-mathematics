from rest_framework.views import APIView
from rest_framework.request import Request
from rest_framework.response import Response
import socket

VENNBASE_HOST='0.0.0.0'
VENNBASE_PORT=1834

class RecordsListCreate(APIView):
    def get(self, request: Request):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((VENNBASE_HOST, VENNBASE_PORT))
            s.sendall(b'get mime:text/html')
            s.close()
        return Response({'message': 'GET request received'})

    def post(self, request: Request):
        return Response({'message': 'POST request received'})
