from django.http import Http404
from rest_framework.decorators import api_view
from rest_framework.response import Response
from rest_framework.viewsets import ViewSet
from rest_framework.request import Request
from rest_framework import parsers
from rest_framework import status
from django.forms import ValidationError

from records.models import Record
from records.serializers import RecordModelSerializer, RecordSerializer
from venndriver.protocol import get_record_by_id

@api_view(['GET'])
def retrieve_record(_: Request, vennbase_id: str):
    try:
        record = Record.objects.get(vennbase_id=vennbase_id)
    except Record.DoesNotExist:
        raise Http404
    except ValidationError:
        return Response({
            'error': f'Vennbase id {vennbase_id} is not a valid uuid'
        }, status=status.HTTP_400_BAD_REQUEST)

    data, mimetype = get_record_by_id(record.vennbase_id)
    print(len(data), mimetype)
    response = Response(
        headers={
            'Content-Disposition': f'attachment; filename={record.name}',
            'Content-Type': mimetype
        }
    )
    response.content = data
    return response

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
