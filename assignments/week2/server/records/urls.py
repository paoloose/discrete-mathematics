from django.urls import path
from records.views import RecordRetrieve, RecordViewSet

urlpatterns = [
    path('', RecordViewSet.as_view({ 'get': 'list', 'post': 'create' }), name='record-list'),
    # path('create/', RecordCreate.as_view({ 'post': 'create' }), name='record-create'),
    # Retrievers a record from Vennbase with its corresponding content-type
    path('<str:vennbase_id>', RecordRetrieve.as_view(), name='record-retrieve')
]
