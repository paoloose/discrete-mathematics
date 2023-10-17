#!/usr/bin/env python
import load as _

from records.views import RecordSerializer, Record
from records.models import RecordTag

# RecordTag.objects.create(name="anime")
# RecordTag.objects.create(name="pink")
tags = RecordTag.objects.all()

data = {
    "name": "123456789012345678901234",
    "record_id": "130cb2cc-3311-4798-aa7d-5f3f56ef2819",
    "tags": []
}

serializer = RecordSerializer(data=data)
serializer.is_valid(raise_exception=True)
serializer.save()

# push record tags to the newly created record
record: Record = serializer.instance
record.tags.add(*tags)
record.tags.add(1)
record.tags.add(1)
record.tags.add(1)
record.tags.add(1)
# bulk means whether to use bulk_create or not, that is, whether to use
# transaction or not. Transacation in this case is not necessary because
# we are not creating a lot of records at once.
