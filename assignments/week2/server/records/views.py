from rest_framework.generics import RetrieveAPIView, ListCreateAPIView

from .models import Record, RecordTag
from .serializers import RecordSerializer

VENNBASE_HOST='0.0.0.0'
VENNBASE_PORT=1834

class RecordRetrieve(RetrieveAPIView):
    queryset = Record.objects.all()
    lookup_field = 'vennbasae_id'

class RecordList(ListCreateAPIView):
    queryset = Record.objects.all()
    serializer_class = RecordSerializer
