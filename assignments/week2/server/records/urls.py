from django.urls import path
from .views import RecordRetrieve, RecordList

urlpatterns = [
    path('', RecordList.as_view(), name='record-list'),
    # Retrievers a record from Vennbase with its corresponding content-type
    path('<str:record_id>', RecordRetrieve.as_view(), name='record-retrieve')
]
