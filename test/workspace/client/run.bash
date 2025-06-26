#!/bin/bash

export PATH=${PATH}:/usr/local/bin/hakoniwa
if [ `uname` = "Darwin" ]
then
    export DYLD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib/hakoniwa
else
    export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib/hakoniwa
fi
export PYTHONPATH="/usr/local/lib/hakoniwa:$PYTHONPATH"
export PYTHONPATH="/usr/local/lib/hakoniwa/py:$PYTHONPATH"
CLIENT_PATH=../hakoniwa-conductor/main/target/debug

if [ $# -eq 1 ]
then
    CLIENT_CUSTOM_JSON_PATH=spec/custom_mqtt.json
    CONDUCTOR_JSON_PATH=client/conductor_config_mqtt.json
else
    CLIENT_CUSTOM_JSON_PATH=spec/custom.json
    CONDUCTOR_JSON_PATH=client/conductor_config.json
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


echo "ACTIVATING CONDUCTOR(CLIENT)"
${CLIENT_PATH}/hakoniwa-conductor-client \
    ${CONDUCTOR_JSON_PATH} \
    ${CLIENT_CUSTOM_JSON_PATH} &
PID_CONDUCTOR=$!

sleep 1

echo "ACTIVATING PYTHON PROG"
python3 client/asset-client-tester.py spec/asset-pdudef.json &
PID_PYTHON=$!

sleep 1
while true
do
    sleep 1
done