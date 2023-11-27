from django.http import Http404
from rest_framework.decorators import api_view
from rest_framework.response import Response
from rest_framework.viewsets import ViewSet
from rest_framework.request import Request
from rest_framework import status
from django.forms import ValidationError
import jsons

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
        queryset = Record.objects.all()
        print(queryset)
        serializer_class = RecordModelSerializer(queryset, many=True)
        for record in serializer_class.data:
            record['tags'] = map(lambda t: t['name'], record['tags'])
        return Response(serializer_class.data)

    def create(self, request: Request):
        serializer = RecordSerializer(data=request.data) # type: ignore
        # ignore error if an unique field already exists
        if serializer.is_valid():
            # This also saves the newly created record to vennbase
            serializer.save()
            return Response(serializer.data, status=status.HTTP_201_CREATED)
        print(serializer.error_messages, serializer.errors)
        return Response(
            serializer.errors,
            status=status.HTTP_400_BAD_REQUEST
        )

    def retrieve(self, request: Request):
        query = request.data.get('query', None)
        if query:
            try:
                records = query_vennbase(query)
                print(records, query)
            except ValueError:
                return Response([], status=status.HTTP_400_BAD_REQUEST)
            # queryset = Record.objects.filter(vennbase_id__in=uuids)
            return Response(
                jsons.dump(records),
                status=status.HTTP_200_OK
            )
        return Response([], status=status.HTTP_400_BAD_REQUEST)
