from rest_framework import serializers

from records.models import Record, RecordTag
from venndriver.protocol import save_record_to_vennbase

class RecordTagSerializer(serializers.ModelSerializer):
    class Meta:
        model = RecordTag
        fields = ['id', 'name']

class RecordModelSerializer(serializers.ModelSerializer):
    class Meta:
        model = Record
        fields = ['id', 'vennbase_id', 'name', 'mimetype', 'tags']

# <https://www.django-rest-framework.org/api-guide/serializers/#serializers>
class RecordSerializer(serializers.Serializer):
    """
    I learnt that in the base serializers.Serializer, you'll need to implement
    the whole set of validation fields! That is, you'll have to satisfy all the
    database field constraints.

    Note that this has nothing to do with the actual Record model. We are just
    mimicking the model fields here.

    A serializer defines HOW a model is serialized. Conceptually, it also wraps
    the model validation from an developer perspective.
    """
    id = serializers.IntegerField(read_only=True)
    mimetype = serializers.CharField(max_length=255)
    # This will be sent to Vennbase and the returned id will be saved to the Record model
    base64_record = serializers.CharField(write_only=True)
    name = serializers.CharField(max_length=100)
    # Serializers itself are also a type of Field!!!
    # Here is when the concept of Relations comes into play!!!
    # tags are many to many field
    tags = RecordTagSerializer(many=True)

    def validate_tags(self, tags):
        print("tags: ", tags)
        if len(tags) > 100:
            raise serializers.ValidationError("tags is too long!")
        return tags

    def create(self, validated_data):
        """
        If not defined, it returns a NotImplementedError: `create()` must be
        implemented.
        """
        uuid = save_record_to_vennbase(
            validated_data['base64_record'],
            validated_data['mimetype']
        )
        record = Record.objects.create(
            vennbase_id=uuid,
            name=validated_data['name'],
            mimetype=validated_data['mimetype']
        )

        tags = validated_data.pop('tags')
        for tag in tags:
            record.tags.add(tag['id'])

        return record

    def update(self, instance: Record, validated_data):
        """
        If not defined, it returns a NotImplementedError: `update()` must be
        implemented.
        """
        instance.name = validated_data.get('name', instance.name)
        instance.save()
        return instance
