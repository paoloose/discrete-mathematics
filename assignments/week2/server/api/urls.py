from django.urls import path, include
from api.views import api_home

urlpatterns = [
    path('', api_home, name='api-home'),
    path('records/', include('records.urls'))
]
