ARG PROJECT=ascii_arcade

FROM rust:alpine3.22 AS build

RUN apk update && apk add --no-cache musl-dev

COPY . .

RUN cargo build --release

FROM alpine:3.22 AS release
ARG PROJECT

COPY --from=build /target/release/${PROJECT} /bin/run

ENTRYPOINT ["/bin/run"]

# REFERENCES
# - https://kerkour.com/rust-docker-from-scratch
