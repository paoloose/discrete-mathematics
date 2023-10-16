from rest_framework.views import APIView
from rest_framework.generics import RetrieveAPIView
from rest_framework.request import Request
from rest_framework.response import Response
import socket

VENNBASE_HOST='0.0.0.0'
VENNBASE_PORT=1834

class RecordRetrieve(RetrieveAPIView):
    ...
