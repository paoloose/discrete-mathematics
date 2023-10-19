#!/bin/bash

set -e

manage_script=./manage.py

if [ ! -f $manage_script ] || [ ! -x $manage_script ]; then
    echo "manage.py not found or not executable"
    exit 1
fi

rm -rf ./db.sqlite3
./manage.py makemigrations
./manage.py makemigrations records
./manage.py migrate
