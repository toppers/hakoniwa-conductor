#!/bin/bash

if [ ! -d workspace/client/for_test/backup ]
then
    echo "ERROR: Can not found dir workspace/client/for_test/backup"
    exit 1
fi

cp /usr/local/lib/hakoniwa/py/hako_robomodel_any.py workspace/client/for_test/backup/
cp /usr/local/lib/hakoniwa/py/hako.py workspace/client/for_test/backup/

cp workspace/client/for_test/hako_robomodel_any.py /usr/local/lib/hakoniwa/py/hako_robomodel_any.py
cp workspace/spec/hako.py /usr/local/lib/hakoniwa/py/hako.py
