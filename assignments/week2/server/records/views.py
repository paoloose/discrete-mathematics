from rest_framework import status
from rest_framework.response import Response
from rest_framework.request import Request
from rest_framework.viewsets import ViewSet
from rest_framework.generics import RetrieveAPIView, ListCreateAPIView
from rest_framework import parsers

from .models import Record
from .serializers import RecordModelSerializer, RecordSerializer

class RecordRetrieve(RetrieveAPIView):
    queryset = Record.objects.all()
    lookup_field = 'vennbase_id'

class RecordViewSet(ViewSet):

    def list(self, _: Request):
        queryset = Record.objects.all()
        serializer_class = RecordModelSerializer(queryset, many=True)
        return Response(serializer_class.data)

    def create(self, request: Request):
        serializer = RecordSerializer(data=request.data) # type: ignore
        if serializer.is_valid():
            serializer.save()
            return Response(serializer.data, status=status.HTTP_201_CREATED)
        return Response(
            serializer.errors,
            status=status.HTTP_400_BAD_REQUEST
        )

    parser_classes = [parsers.JSONParser, parsers.MultiPartParser]
