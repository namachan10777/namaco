FROM rust

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y cmake curl g++ jq python binutils-dev libcurl4-openssl-dev zlib1g-dev libdw-dev libiberty-dev valgrind

ENTRYPOINT ["/bin/bash"]
