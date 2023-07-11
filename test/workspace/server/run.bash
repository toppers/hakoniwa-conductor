#!/bin/bash

hako-master-rust ${DELTA_MSEC} ${MAX_DELAY_MSEC} \
    ${CORE_IPADDR}:${GRPC_PORT} ${UDP_SRV_PORT} ${UDP_SND_PORT} &

sleep 1

LAST_PID=
PYTHON_PROG=server/asset-srv-tester.py
echo "INFO: ACTIVATING :${PYTHON_PROG}"
python3 ${PYTHON_PROG} &
LAST_PID=$!
sleep 1

while [ 1 ]
do
    sleep 10
done
