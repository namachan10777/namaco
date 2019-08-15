FROM rust

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y cmake curl g++ jq python binutils-dev libcurl4-openssl-dev zlib1g-dev libdw-dev libiberty-dev

RUN cargo install cargo-kcov && \
	cargo kcov --print-install-kcov-sh | sh

ENTRYPOINT ["/bin/bash"]
