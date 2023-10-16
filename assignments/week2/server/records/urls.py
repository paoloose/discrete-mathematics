from django.urls import path
from .views import RecordRetrieve

urlpatterns = [
    # Retrievers a record from Vennbase with its corresponding content-type
    path('<str:record_id>', RecordRetrieve.as_view(), name='record-retrieve')
]
