FROM ubuntu

ENV DEBIAN_FRONTEND noninteractive

# https://askubuntu.com/questions/16225/how-can-i-accept-the-microsoft-eula-agreement-for-ttf-mscorefonts-installer
#RUN echo "ttf-mscorefonts-installer msttcorefonts/accepted-mscorefonts-eula select true" | debconf-set-selections \
#    && echo "ttf-mscorefonts-installer msttcorefonts/present-mscorefonts-eula note" | debconf-set-selections

RUN apt-get update
RUN apt-get install -y fontconfig
RUN echo "ttf-mscorefonts-installer msttcorefonts/accepted-mscorefonts-eula select true" | debconf-set-selections \
     && DEBIAN_FRONTEND=noninteractive\
        apt-get -y install\
    	ttf-mscorefonts-installer

# deps and nodejs
RUN apt-get install -y \
        curl \
        software-properties-common \
        gcc \
        clang \
        pkg-config \
        libssl-dev \
        python3 \
        g++ \
        git \
        make \
        cmake \
        libfreetype6-dev \
        libfontconfig1-dev \
        xclip \
        liblmdb0 liblmdb-dev \
    && fc-cache -f \
    && curl -fsSL https://deb.nodesource.com/setup_16.x | bash - \
    && apt-get install -y nodejs

# Rust and wasi
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && . "$HOME/.cargo/env" \
    && rustup target add wasm32-wasi \
    && curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

# Elm
RUN curl -L -o elm.gz https://github.com/elm/compiler/releases/download/0.19.1/binary-for-linux-64-bit.gz \
    && gunzip elm.gz \
    && chmod +x elm \
    && mv elm /usr/local/bin/