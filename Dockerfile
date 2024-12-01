# Author: FL03 <jo3mccain@icloud.com>
ARG RUST_VERSION=latest

FROM rust:${RUST_VERSION} AS builder-base

# update and upgrade the system
RUN apt-get update -y && apt-get upgrade -y

FROM builder-base AS builder
# setup the working directory
WORKDIR /app
# copy the source code
ADD . .
# build the project
RUN --mount=type=cache,target=/workspace/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release --verbose

FROM debian:bookworm-slim AS runner-base

# update and upgrade the system
RUN apt-get update -y && \
    apt-get upgrade -y

# install the required system dependencies
RUN apt-get install -y \
    postgresql

# create a user and group
RUN groupadd -g 10001 agroup && \
    useradd -m -u 10001 -g agroup auser

# switch to the user
USER auser

# copy the binary to the system
COPY --from=builder --chown=auser:agroup /app/target/release/served /usr/local/bin/served
# copy the configuration files
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/.config /opt/served/.config
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/*.config.toml* /opt/served/.config/*.config.toml*
# set the permissions
RUN chmod +x /usr/local/bin/served && \
    chmod +x /opt/served/.config && \
    chown auser /usr/local/bin/served && \
    chown -R auser /opt/served

FROM runner-base AS runner
# Set the environment variables
ENV APP_MODE=release \
    HOSTNAME="0.0.0.0" \
    HOSTPORT=8080 \
    PZZLD_CONFIG="/opt/served/.config" \
    PZZLD_WORKDIR="/opt/served" \
    RUST_LOG=trace
# set the working directory
WORKDIR /opt/served
# expose the port
EXPOSE ${HOSTPORT}
# set the entrypoint
ENTRYPOINT ["served"]
