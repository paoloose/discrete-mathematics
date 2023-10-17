from rest_framework import serializers

from .models import Record, RecordTag

class RecordTagSerializer(serializers.ModelSerializer):
    class Meta:
        model = RecordTag
        fields = ['id', 'name']

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
    vennbase_id = serializers.UUIDField()
    name = serializers.CharField(max_length=100)
    # Resializers itself are also a type of Field!!!
    # Here is when the concept of Relations comes into play!!!
    # tags are many to many field
    tags = RecordTagSerializer(many=True)

    def validate_name(self, value):
        if len(value) > 100:
            raise serializers.ValidationError("name is too long!")
        return f"{value}!"

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
        new_record = Record(
            vennbasae_id=validated_data['vennbasae_id'],
            name=validated_data['name']
        )
        for tag in validated_data['tags']:
            new_record.tags.add(tag)
        new_record.save()
        return new_record

    def update(self, instance: Record, validated_data):
        """
        If not defined, it returns a NotImplementedError: `update()` must be
        implemented.
        """
        instance.name = validated_data.get('name', instance.name)
        instance.save()
        return instance
