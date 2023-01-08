FROM rust as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN rustup update

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --release -v --workspace

FROM debian:buster-slim as runner-base

ENV RUST_LOG="info" \
    SERVER_PORT=8080

RUN apt-get update -y && apt-get upgrade -y

FROM runner-base as runner 

COPY --chown=55 .config /config
VOLUME [ "/config" ]

COPY --from=builder /workspace/target/release/app /bin/app

FROM runner

EXPOSE 80
EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "app" ]
CMD [ "--up" ]
