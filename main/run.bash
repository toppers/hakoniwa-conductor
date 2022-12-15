#!/bin/bash

export LD_LIBRARY_PATH=/usr/local/lib/hakoniwa

./target/debug/main ${1} ${2}
