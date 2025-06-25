#!/bin/bash

IMAGE_NAME=`cat docker/client/image_name.txt`
IMAGE_TAG=`cat docker/client/appendix/latest_version.txt`
DOCKER_IMAGE=toppersjp/${IMAGE_NAME}:${IMAGE_TAG}
DOCKER_FILE=docker/client/Dockerfile
if [ `uname` = "Darwin" ]; then
    docker build --platform linux/amd64 -t ${DOCKER_IMAGE} -f ${DOCKER_FILE} .
else
    docker build -t ${DOCKER_IMAGE} -f ${DOCKER_FILE} .
fi
