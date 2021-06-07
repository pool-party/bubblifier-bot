FROM rust:1.51

WORKDIR /usr/src/app
COPY . .
WORKDIR /usr/src/app/api

RUN cargo install --path .
