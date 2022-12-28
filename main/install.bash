#!/bin/bash

HAKO_CORE_PATH=../../hakoniwa-core-cpp-client

if [ -d ${HAKO_CORE_PATH} ]
then
	:
else
	echo "ERROR: not found hakoniwa core path on ${HAKO_CORE_PATH}"
	echo "Please git clone https://github.com/toppers/hakoniwa-core-cpp-client.git"
	echo "$ cd hakoniwa-core-cpp-client"
	echo "$ bash build.bash clean"
	echo "$ bash build.bash"
	exit 1
fi

OS_TYPE=`uname`
EXT_NAME=so
if [ $OS_TYPE = "Linux" ]
then
	:
elif [ $OS_TYPE = "Darwin" ]
then
	EXT_NAME=dylib
else
	echo "ERROR: not supported OS(`uname`)"
	exit 1
fi

HAKO_CORE_LIB=${HAKO_CORE_PATH}/cmake-build/src/hakoc/libshakoc.${EXT_NAME}
HAKO_LIB_DIR=/usr/local/lib/hakoniwa
HAKO_BIN_DIR=/usr/local/bin/hakoniwa
if [ -d ${HAKO_LIB_DIR} ]
then
	echo "PASSED: found ${HAKO_LIB_DIR}"
else
	echo "ERROR: not found hakoniwa library dir: ${HAKO_LIB_DIR}"
	echo "Please mkdir ${HAKO_LIB_DIR}"
	exit 1
fi
if [ -d ${HAKO_BIN_DIR} ]
then
	echo "PASSED: found ${HAKO_BIN_DIR}"
else
	echo "ERROR: not found hakoniwa bin dir: ${HAKO_BIN_DIR}"
	echo "Please mkdir ${HAKO_BIN_DIR}"
	exit 1
fi

if [ -f ${HAKO_LIB_DIR}/libshakoc.${EXT_NAME} ]
then
	echo "PASSED: installed ${HAKO_LIB_DIR}/libshakoc.${EXT_NAME}"
else
	echo "ERROR: not found hakoniwa library: ${HAKO_LIB_DIR}/libshakoc.${EXT_NAME}"
	echo "Please cp ${HAKO_CORE_LIB} ${HAKO_LIB_DIR}/libshakoc.${EXT_NAME}"
	exit 1
fi
which protoc > /dev/null
if [ $? -ne 0 ]
then
	echo "ERROR: not found protoc"
	echo "Please install protoc"
	if [ $OS_TYPE = "Linux" ]
	then
		echo "HINT: sudo apt install protobuf-compiler"
	elif [ $OS_TYPE = "Darwin" ]
	then
		: #TODO
	fi
	exit 1
fi

which cargo > /dev/null
if [ $? -ne 0 ]
then
	echo "ERROR: not found cargo"
	echo "Please install cargo"
	if [ $OS_TYPE = "Linux" ]
	then
		echo "HINT: https://www.aise.ics.saitama-u.ac.jp/~gotoh/RustOnUbuntu2004.html"
		echo "HINT: sudo apt install -y build-essential"
		echo "HINT: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
		echo "HINT: source ${HOME}/.cargo/env"
	elif [ $OS_TYPE = "Darwin" ]
	then
		: #TODO
	fi
	exit 1
else
	echo "PASSED: installed cargo"
fi

#Library check
if [ $OS_TYPE = "Linux" ]
then
	sudo ldconfig -p | grep libc++.so
	if [ $? -ne 0 ]
	then
		echo "FALIED: not found libc++.so"
		echo "Please install libc++ : sudo apt install libc++-dev libc++abi-dev"
		exit 1
	else
		echo "PASSED: found libc++.so"
	fi
	sudo ldconfig -p | grep libc++abi.so
	if [ $? -ne 0 ]
	then
		echo "FALIED: not found libc++abi.so"
		echo "Please install libc++ : sudo apt install libc++-dev libc++abi-dev"
		exit 1
	else
		echo "PASSED: found libc++abi.so"
	fi

elif [ $OS_TYPE = "Darwin" ]
then
	:
fi

echo "OK!!"
echo "Please build hakoniwa-master for rust"
echo "BUILD: cargo build"
cargo build
sudo cp ./target/debug/main ${HAKO_BIN_DIR}/hako-master-rust
sudo cp hako-master ${HAKO_BIN_DIR}/hako-master
sudo cp hako-cleanup ${HAKO_BIN_DIR}/hako-cleanup
sudo chmod +x ${HAKO_BIN_DIR}/hako-master
sudo chmod +x ${HAKO_BIN_DIR}/hako-cleanup

