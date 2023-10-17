from django.db import models

class RecordTag(models.Model):
    id = models.AutoField(primary_key=True)
    name = models.CharField(max_length=100)

class Record(models.Model):
    id = models.AutoField(primary_key=True)
    # Vennbase record id
    vennbase_id = models.UUIDField()
    name = models.CharField(max_length=100)
    # array of tags
    tags = models.ManyToManyField(RecordTag, related_name='records')
