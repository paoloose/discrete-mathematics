from django.db import models

class RecordTag(models.Model):
    name = models.CharField(max_length=100)

class Record(models.Model):
    name = models.CharField(max_length=100)
    value = models.IntegerField()
    # array of tags
    tags = models.ManyToManyField(RecordTag)
