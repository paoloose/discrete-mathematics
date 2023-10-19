# Generated by Django 4.2.6 on 2023-10-19 18:31

from django.db import migrations, models


class Migration(migrations.Migration):

    initial = True

    dependencies = [
    ]

    operations = [
        migrations.CreateModel(
            name='RecordTag',
            fields=[
                ('id', models.AutoField(primary_key=True, serialize=False)),
                ('name', models.CharField(max_length=100)),
            ],
        ),
        migrations.CreateModel(
            name='Record',
            fields=[
                ('id', models.AutoField(primary_key=True, serialize=False)),
                ('vennbase_id', models.UUIDField(unique=True)),
                ('name', models.CharField(max_length=100)),
                ('mimetype', models.CharField(max_length=255)),
                ('tags', models.ManyToManyField(related_name='records', to='records.recordtag')),
            ],
        ),
    ]
