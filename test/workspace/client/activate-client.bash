#!/bin/bash

export PATH=${PATH}:/usr/local/bin/hakoniwa
export LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/local/lib/hakoniwa
CLIENT_PATH=../main/target/debug/


${CLIENT_PATH}/hakoniwa-conductor-client \
    workspace/client/conductor_config.json \
    workspace/client/custom.json

