from django.contrib import admin
from django.urls import path, include

from rest_framework.decorators import api_view
from rest_framework.response import Response

@api_view(['GET'])
def health_view(request):
    import socket
    from venndriver.protocol import ADDR, PORT
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        s.connect((ADDR, PORT))
    except ConnectionRefusedError:
        return Response({'status': 'VENNBASE_DOWN'}, status=500)
    return Response({'status': 'OK'}, status=200)

urlpatterns = [
    path('admin/', admin.site.urls),
    path('api/', include('api.urls')),
    path('health/', health_view)
]
