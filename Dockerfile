# syntax=docker/dockerfile:1.4.2-labs

#==============================================================================
# Builder
#==============================================================================

ARG DEBIAN_VERSION=bullseye
ARG DEBIAN_FLAVOR=slim
ARG RUST_VERSION=1.62

FROM rust:${RUST_VERSION}-${DEBIAN_FLAVOR}-${DEBIAN_VERSION} AS build

RUN apt update && \
    apt upgrade -y && \
    build_deps=' \
        build-essential \
        openssl \
        libssl-dev \
        pkg-config \
    ' && \
   apt-get install -y $build_deps

WORKDIR /opt/app

COPY Cargo.toml Cargo.lock Makefile ./

RUN --mount=type=cache,id=cargo-x11-sentinel-client,sharing=locked,target=/root/.cargo/registry/cache \
    make install-deps

COPY . .

RUN --mount=type=cache,id=cargo-x11-sentinel-client,sharing=locked,target=/root/.cargo/registry/cache \
    make compile

#===============================================================================
# Export
#===============================================================================

FROM scratch AS export

COPY --from=build /opt/app/target/release/x11-sentinel-client /
