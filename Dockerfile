ARG WORKDIR=/home/build

FROM rust:alpine3.22 AS build

ARG PROJECT
ARG WORKDIR
WORKDIR ${WORKDIR}

RUN apk update && apk add --no-cache musl-dev

COPY Cargo.lock Cargo.toml .
COPY src src
COPY tests tests

RUN cargo build --release

FROM alpine:3.22 AS release

ARG PROJECT
ARG WORKDIR
WORKDIR ${WORKDIR}

COPY --from=build ${WORKDIR}/target/release/${PROJECT} /bin/run

CMD ["/bin/run"]

# REFERENCES
# - https://kerkour.com/rust-docker-from-scratch
