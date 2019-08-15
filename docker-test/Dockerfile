FROM rust

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y cmake curl g++ jq python binutils-dev libcurl4-openssl-dev zlib1g-dev libdw-dev libiberty-dev

ENV KCOV_VERSION 33
RUN wget https://github.com/SimonKagstrom/kcov/archive/v$KCOV_VERSION.tar.gz && \
    tar xzf v$KCOV_VERSION.tar.gz && \
    rm v$KCOV_VERSION.tar.gz && \
    cd kcov-$KCOV_VERSION && \
    mkdir build && cd build && \
    cmake .. && make && make install && \
    cd ../.. && rm -rf kcov-$KCOV_VERSION

ENTRYPOINT ["/bin/bash"]
