#!/bin/bash

export PATH=${PATH}:/usr/local/bin/hakoniwa
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib/hakoniwa
export PYTHONPATH="/usr/local/lib/hakoniwa:$PYTHONPATH"
export PYTHONPATH="/usr/local/lib/hakoniwa/py:$PYTHONPATH"
CLIENT_PATH=../../main/target/debug/

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
    client/conductor_config.json \
    spec/custom.json &
PID_CONDUCTOR=$!

sleep 1

echo "ACTIVATING PYTHON PROG"
python3 client/asset-client-tester.py &
PID_PYTHON=$!

sleep 1
while true
do
    sleep 1
done