#!/bin/bash

export PATH=${PATH}:/usr/local/bin/hakoniwa
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib/hakoniwa
export PYTHONPATH="/usr/local/lib/hakoniwa:$PYTHONPATH"
export PYTHONPATH="/usr/local/lib/hakoniwa/py:$PYTHONPATH"
CLIENT_PATH=../../main/target/debug/

CLIENT_CUSTOM_JSON_PATH=spec/custom.json
if [ $# -eq 1 ]
then
    CLIENT_CUSTOM_JSON_PATH=$1
fi

#PIQ_MOSQUITO=
PID_CONDUCTOR=
PID_PYTHON=
function handle_signal {
    echo "RECV SIGNAL"
    if [ ! -z $PID_CONDUCTOR ]
    then
        kill -9 $PID_CONDUCTOR
    fi
    if [ ! -z $PID_PYTHON ]
    then
        kill -9 $PID_PYTHON
    fi
#    if [ ! -z $PIQ_MOSQUITO ]
#    then
#        kill -9 $PIQ_MOSQUITO
#    fi
    echo "WAITKING KILLED PROCS"
    sleep 1
    hako-cleanup
    echo "IPCS INFO"
    ipcs
    exit 0
}
trap handle_signal SIGINT SIGTERM


#MQTT_PORT=
#grep MQTT $CLIENT_CUSTOM_JSON_PATH > /dev/null
#if [ $? -eq 0 ]
#then
#    which mosquitto > /dev/null
#    if [ $? -ne 0 ]
#    then
#        echo "ERROR: Please install mosquitto"
#        echo "sudo apt install -y mosquitto mosquitto-clients"
#        exit 1
#    fi
#    cd client
#    MQTT_PORT=1983
#    echo "INFO: ACTIVATING MOSQUITTO"
#    mosquitto -c config/mosquitto.conf &
#    PID_CONDUCTOR=$!
#    sleep 2
#    cd ..
#fi

echo "ACTIVATING CONDUCTOR(CLIENT)"
${CLIENT_PATH}/hakoniwa-conductor-client \
    client/conductor_config.json \
    ${CLIENT_CUSTOM_JSON_PATH} &
PID_CONDUCTOR=$!

sleep 1

echo "ACTIVATING PYTHON PROG"
python3 client/asset-client-tester.py ${CLIENT_CUSTOM_JSON_PATH} &
PID_PYTHON=$!

sleep 1
while true
do
    sleep 1
done