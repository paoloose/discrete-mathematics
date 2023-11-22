from rest_framework import serializers

from records.models import Record, RecordTag
from venndriver.protocol import save_record_to_vennbase

class RecordTagSerializer(serializers.ModelSerializer):
    class Meta:
        model = RecordTag
        fields = ['id', 'name']

class RecordModelSerializer(serializers.ModelSerializer):
    id = serializers.UUIDField(source='vennbase_id')

    class Meta:
        model = Record
        depth = 1
        # when returning the validated data, i want the 'vennbase_id' to be called just 'id'
        fields = ['id', 'name', 'mimetype', 'tags']

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
        if len(tags) > 100:
            raise serializers.ValidationError("tags is too long!")
        return tags

    def create(self, validated_data):
        """
        If not defined, it returns a NotImplementedError: `create()` must be
        implemented.
        """
        tags = list(map(lambda x: x['name'], validated_data['tags']))
        uuid = save_record_to_vennbase(
            validated_data['base64_record'],
            validated_data['mimetype'],
            tags
        )
        record = Record.objects.create(
            vennbase_id=uuid,
            name=validated_data['name'],
            mimetype=validated_data['mimetype']
        )

        # we've already sent the tags to vennbase, now we need to
        # sync the tags with the database
        tags = validated_data.pop('tags')
        # TODO: can this be optimized?
        # bulk FIND the tags that match with the list of names
        for tag in tags:
            tag, _ = RecordTag.objects.get_or_create(name=tag['name'])
            record.tags.add(tag)

        return record

    def update(self, instance: Record, validated_data):
        """
        If not defined, it returns a NotImplementedError: `update()` must be
        implemented.
        """
        instance.name = validated_data.get('name', instance.name)
        instance.save()
        return instance
