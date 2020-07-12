FROM rust:1.44.1

WORKDIR /usr/src/minigrep
COPY . .

RUN cargo install --path .