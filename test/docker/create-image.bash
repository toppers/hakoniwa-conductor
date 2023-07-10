#!/bin/bash

IMAGE_NAME=`cat docker/image_name.txt`
IMAGE_TAG=`cat docker/appendix/latest_version.txt`
DOCKER_IMAGE=toppersjp/${IMAGE_NAME}:${IMAGE_TAG}
DOCKER_FILE=docker/Dockerfile
docker build -t ${DOCKER_IMAGE} -f ${DOCKER_FILE} .
