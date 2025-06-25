#!/bin/bash

if [ `uname` = "Darwin" ]; then
    export DYLD_LIBRARY_PATH=/usr/local/lib/hakoniwa
else
    export LD_LIBRARY_PATH=/usr/local/lib/hakoniwa
fi

./target/debug/main ${1} ${2} ${3}
