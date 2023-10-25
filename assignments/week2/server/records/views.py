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
from venndriver.protocol import get_record_by_id, query_vennbase

@api_view(['GET'])
def retrieve_record(req: Request, vennbase_id: str):
    try:
        record = Record.objects.get(vennbase_id=vennbase_id)
    except Record.DoesNotExist:
        raise Http404
    except ValidationError:
        return Response({
            'error': f'Vennbase id {vennbase_id} is not a valid uuid'
        }, status=status.HTTP_400_BAD_REQUEST)

    data, mimetype = get_record_by_id(
        record.vennbase_id,
        resize=req.GET.get('resize', None)
    )
    print(len(data), mimetype)
    response = Response(
        headers={
            # 'Content-Disposition': f'attachment; filen ame={record.name}',
            'Content-Type': mimetype,
            'Cache-Control': 'max-age=31536000',
        }
    )
    response.content = data
    return response

class RecordViewSet(ViewSet):

    def list(self, request: Request):
        query = request.GET.get('query', '')
        if query:
            try:
                uuids = query_vennbase(query)
            except ValueError:
                return Response([], status=status.HTTP_400_BAD_REQUEST)
            queryset = Record.objects.filter(vennbase_id__in=uuids)
        else:
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
