#!/bin/bash

if [ -z ${CLIENT_CUSTOM_JSON_PATH} ]
then
    CLIENT_CUSTOM_JSON_PATH=spec/custom.json
fi

hako-master-rust ${DELTA_MSEC} ${MAX_DELAY_MSEC} \
    ${CORE_IPADDR}:${GRPC_PORT} ${UDP_SRV_PORT} ${UDP_SND_PORT} &

sleep 1


LAST_PID=
cp spec/hako.py /usr/local/lib/hakoniwa/py/hako.py
PYTHON_PROG=server/asset-srv-tester.py
echo "INFO: ACTIVATING :${PYTHON_PROG}"
python3 ${PYTHON_PROG} ${CLIENT_CUSTOM_JSON_PATH} &
LAST_PID=$!
sleep 1

while [ 1 ]
do
    sleep 10
done
