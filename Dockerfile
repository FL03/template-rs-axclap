# Author: FL03 <jo3mccain@icloud.com>
# Date: 2025-05-12
# Description: Dockerfile for templated
# License: Apache-2.0

# Current Rust version: 
ARG RUST_VERSION=1.86.0 \ 
    APP_VERSION=0.0.0
# STAGE 1: create the base image for the build stage(s)
FROM rust:${RUST_VERSION} AS builder-base
# update and upgrade the image packages
RUN apt-get update -y && apt-get upgrade -y
# STAGE 1.1: separate actual build-logic from the base image
FROM builder-base AS builder
# declare environment variables for the build stage
ENV RUST_BACKTRACE=1 \
    RUST_LOG="templated=debug,info"
# switch the working directory
WORKDIR /app
# include all relevant sources
ADD . .
# build the project
RUN --mount=type=cache,target=/workspace/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release --workspace --features full
# STAGE 2: setup the runtime image
FROM debian:bookworm-slim AS runner-base
# update and upgrade the system
RUN apt-get update -y && apt-get upgrade -y
# install any required runtime dependencies
# RUN apt-get install -y postgresql libsqlite3-dev
# create a user and group
RUN groupadd -g 10001 agroup && \
    useradd -m -u 10001 -g agroup auser
# switch to the new user
USER auser
# copy the binary to the system
COPY --from=builder --chown=auser:agroup /app/target/release/templated /usr/local/bin/templated
# copy the configuration directory
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/.config /opt/templated/.config
# copy any additional configuration files that may be located at the root of the project
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/*.config.toml* /opt/templated/.config/*.config.toml*
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/templated.toml* /opt/templated/.config/templated.toml*
# set the permissions
RUN chmod +x /usr/local/bin/templated && \
    chmod +x /opt/templated/.config && \
    chown auser /usr/local/bin/templated && \
    chown -R auser /opt/templated
# STAGE 2.2 setup the final stage optimized to run the built application
FROM runner-base AS runner
# Set the environment variables
ENV APP_MODE=release \
    APP_HOST="0.0.0.0" \
    APP_PORT=8080 \
    APP_CONFIG_DIR="/opt/templated/.config" \
    RUST_LOG="templated=debug,info"
# set the working directory
WORKDIR /opt/templated
# expose the port
EXPOSE ${APP_PORT}
# set the entrypoint
ENTRYPOINT ["templated"]