FROM docker.io/rust:1-slim-bookworm AS rocket-build

RUN apt-get update && apt-get install -y libpq-dev

## cargo package name: customize here or provide via --build-arg
ARG pkg=farme-rs

WORKDIR /build

COPY Cargo.toml .
COPY Cargo.lock .
COPY server server
COPY database database

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

################################################################################
FROM node:alpine AS ng-build

RUN npm install -g @angular/cli

WORKDIR /build

COPY web .

RUN npm install && ng build

################################################################################

FROM docker.io/debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /app

## copy the main binary
COPY --from=rocket-build /build/main ./
COPY --from=ng-build /build/dist/farmers/browser ./browser

## ensure the container listens globally on port 8080
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV WEBAPP_PATH=/app/browser

CMD ./main
