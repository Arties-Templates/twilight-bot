FROM rust:slim as installer

WORKDIR /installer
SHELL ["/bin/bash", "-c"]

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install wget build-essential make gcc pkg-config libssl-dev --no-install-recommends -y

RUN wget -O /installer/dumb-init https://github.com/Yelp/dumb-init/releases/download/v1.2.5/dumb-init_1.2.5_x86_64
RUN chmod +x /installer/dumb-init

FROM installer as builder

WORKDIR /builder
SHELL ["/bin/bash", "-c"]

COPY . .
RUN cargo build --release

FROM debian:11-slim

WORKDIR /runner

COPY --from=installer /installer/dumb-init /usr/local/bin
COPY --from=builder /builder/target/release/xtremefx .
SHELL ["/bin/bash", "-c"]

ENTRYPOINT ["/usr/local/bin/dumb-init", "--"]
CMD ["/runner/xtremefx"]
