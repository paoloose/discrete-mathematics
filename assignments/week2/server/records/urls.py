from django.urls import path
from .views import RecordsListCreate

urlpatterns = [
    path('', RecordsListCreate.as_view(), name='articles-list-create')
]
