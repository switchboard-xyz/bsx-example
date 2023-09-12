# syntax=docker/dockerfile:1.4
FROM switchboardlabs/sgx-function:main AS builder

ARG CARGO_NAME=switchboard-function-evm
ENV CARGO_NAME=$CARGO_NAME
ARG EXAMPLE_PROGRAM
ENV EXAMPLE_PROGRAM=$EXAMPLE_PROGRAM

WORKDIR /home/root/switchboard-function/sgx-function
COPY ./Cargo.toml  ./
COPY ./build.rs  ./
COPY ./src ./src

# RUN rustup update
#
# RUN rustup override set stable
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=target,id=${TARGETPLATFORM} \
    cargo build --release && \
    cargo strip && \
    mv target/release/basic-oracle-function /sgx/app

FROM switchboardlabs/sgx-function:main

# Copy the binary
WORKDIR /sgx
COPY --from=builder /sgx/app /sgx
COPY ./config/app.manifest.template /sgx/app.manifest.template


# Get the measurement from the enclave
RUN /get_measurement.sh

# COPY ./config/boot.sh /boot.sh
ENTRYPOINT ["bash", "/boot.sh"]
