#!/bin/bash

export DELTA_MSEC=20
export MAX_DELAY_MSEC=20
export GRPC_PORT=50051
export UDP_SRV_PORT=54001
export UDP_SND_PORT=54002
export CORE_IPADDR="172.20.0.10"

if [ -z ${CLIENT_CUSTOM_JSON_PATH} ]
then
    CLIENT_CUSTOM_JSON_PATH=spec/asset-pdudef.json
fi
MQTT_PORT=
grep MQTT $CLIENT_CUSTOM_JSON_PATH > /dev/null
if [ $? -eq 0 ]
then
    which mosquitto > /dev/null
    if [ $? -ne 0 ]
    then
        echo "ERROR: Please install mosquitto"
        exit 1
    fi
    cd server
    MQTT_PORT=1883
    echo "INFO: ACTIVATING MOSQUITTO"
    mosquitto -c config/mosquitto.conf &
    sleep 2
    cd ..
fi

hako-master-rust ${DELTA_MSEC} ${MAX_DELAY_MSEC} \
    ${CORE_IPADDR}:${GRPC_PORT} ${UDP_SRV_PORT} ${UDP_SND_PORT} \
    ${MQTT_PORT} &

sleep 1


LAST_PID=
PYTHON_PROG=server/asset-srv-tester.py
echo "INFO: ACTIVATING :${PYTHON_PROG}"
python3 ${PYTHON_PROG} ${CLIENT_CUSTOM_JSON_PATH} &
LAST_PID=$!
sleep 1

while [ 1 ]
do
    sleep 10
done
