#!/bin/bash

export HOST_WORKDIR=`pwd`/workspace
export HOST_DEVDIR=`pwd`/workspace/server
export HOST_SPECDIR=`pwd`/workspace/spec
export DOCKER_DIR=/root/workspace
export DOCKER_DEVDIR=/root/workspace/server
export DOCKER_SPECDIR=/root/workspace/spec

export DELTA_MSEC=20
export MAX_DELAY_MSEC=100
export GRPC_PORT=50051
export UDP_SRV_PORT=54001
export UDP_SND_PORT=54002
