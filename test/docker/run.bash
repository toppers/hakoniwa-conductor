#!/bin/bash

source docker/env.bash

CLIENT_CUSTOM_JSON_PATH=spec/custom.json
if [ $# -eq 1 ]
then
	CLIENT_CUSTOM_JSON_PATH=$1
fi

HAKONIWA_TOP_DIR=`pwd`
IMAGE_NAME=`cat docker//image_name.txt`
IMAGE_TAG=`cat docker/appendix/latest_version.txt`
DOCKER_IMAGE=toppersjp/${IMAGE_NAME}:${IMAGE_TAG}

OS_TYPE=`bash docker/utils/detect_os_type.bash`

if [ ${OS_TYPE} != "Mac" ]
then
	docker ps > /dev/null
	if [ $? -ne 0 ]
	then
	    sudo service docker start
	    echo "waiting for docker service activation.. "
	    sleep 3
	fi
fi

if [ ${OS_TYPE} = "wsl2" ]
then
	export RESOLV_IPADDR=`cat /etc/resolv.conf  | grep nameserver | awk '{print $NF}'`
	NETWORK_INTERFACE=$(route | grep '^default' | grep -o '[^ ]*$' | tr -d '\n')
	CORE_IPADDR=$(ifconfig "${NETWORK_INTERFACE}" | grep netmask | awk '{print $2}')
elif [ ${OS_TYPE} = "Mac" ]
then
	if [ $# -ne 1 ]
	then
		echo "Usage: $0 <port>"
		exit 1
	fi
	#NETWORK_INTERFACE=$(netstat -rnf inet | grep '^default' | awk '{print $4}')
	#CORE_IPADDR=$(ifconfig "${NETWORK_INTERFACE}" | grep netmask | awk '{print $2}')
	CORE_IPADDR="127.0.0.1"
else
	IPADDR="127.0.0.1"
fi

docker run \
	-v ${HOST_DEVDIR}:${DOCKER_DEVDIR} \
	-v ${HOST_SPECDIR}:${DOCKER_SPECDIR} \
	-v ${HOST_WORKDIR}:${DOCKER_DIR} \
	-it --rm \
	--net host \
	-e CORE_IPADDR=${CORE_IPADDR} \
	-e DELTA_MSEC=${DELTA_MSEC} \
	-e MAX_DELAY_MSEC=${MAX_DELAY_MSEC} \
	-e GRPC_PORT=${GRPC_PORT} \
	-e UDP_SRV_PORT=${UDP_SRV_PORT} \
	-e UDP_SND_PORT=${UDP_SND_PORT} \
	-e CLIENT_CUSTOM_JSON_PATH=${CLIENT_CUSTOM_JSON_PATH} \
	--name ${IMAGE_NAME} ${DOCKER_IMAGE} 

