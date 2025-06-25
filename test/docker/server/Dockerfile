FROM ubuntu:20.04 as hakoniwa-conductor-server

ARG DEBIAN_FRONTEND=noninteractive

ENV CMAKE_INSTALL_DIR /usr/local/cmake
ENV PATH $PATH:/usr/local/bin/hakoniwa
ENV LD_LIBRARY_PATH $LD_LIBRARY_PATH:/usr/local/lib/hakoniwa

RUN apt-get update && apt-get install -y \
	git	\
	build-essential	\
	wget	\
	gcc	\
	g++	\
	ruby	\
	vim	\
	gem \
	libssl-dev libreadline-dev zlib1g-dev \
	make	\
	autoconf \
	automake \
	pkg-config \
	curl \
	net-tools \
	netcat \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/*

RUN wget -q -O cmake-linux.sh https://github.com/Kitware/CMake/releases/download/v3.27.9/cmake-3.27.9-linux-x86_64.sh && \
    mkdir -p $CMAKE_INSTALL_DIR && \
    sh cmake-linux.sh --skip-license --prefix=$CMAKE_INSTALL_DIR && \
    rm cmake-linux.sh
ENV PATH ${PATH}:${CMAKE_INSTALL_DIR}/bin

RUN apt-get update

RUN mkdir /usr/local/lib/hakoniwa
RUN mkdir /usr/local/bin/hakoniwa

RUN apt-get install -y protobuf-compiler
RUN apt-get install -y build-essential

ENV RUST_HOME /usr/local/lib/rust
ENV RUSTUP_HOME ${RUST_HOME}/rustup
ENV CARGO_HOME ${RUST_HOME}/cargo
RUN mkdir /usr/local/lib/rust && \
    chmod 0755 $RUST_HOME
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > ${RUST_HOME}/rustup.sh \
    && chmod +x ${RUST_HOME}/rustup.sh \
    && ${RUST_HOME}/rustup.sh -y --default-toolchain nightly --no-modify-path
ENV PATH $PATH:$CARGO_HOME/bin

RUN apt-get install -y libc++-dev libc++abi-dev

RUN apt install -y mosquitto mosquitto-clients

ENV PYTHONPATH="/usr/local/lib/hakoniwa:$PYTHONPATH"
WORKDIR /root
RUN git clone --recursive https://github.com/toppers/hakoniwa-core-cpp-client.git
WORKDIR /root/hakoniwa-core-cpp-client
RUN apt-get install -y python3-pip  libffi-dev python3-dev
RUN bash build.bash
RUN bash install.bash

ENV PYTHONPATH="/usr/local/lib/hakoniwa/py:$PYTHONPATH"
RUN pip3 install pyquaternion numpy scipy

WORKDIR /root
RUN git clone --recursive https://github.com/toppers/hakoniwa-conductor.git
WORKDIR /root/hakoniwa-conductor/main
RUN bash install.bash

RUN mkdir -p /root/workspace
WORKDIR /root/workspace

#CMD [ "/root/workspace/server/run.bash" ]
