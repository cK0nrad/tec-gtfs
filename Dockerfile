FROM rust:1.74.1-slim

# Path: /usr/src/app
WORKDIR /usr/src/app

# Path: /usr/src/app/Cargo.toml
COPY Cargo.toml .

# Path: /usr/src/app/Cargo.lock
COPY Cargo.lock .

# Path: /usr/src/app/src
COPY src src

RUN apt-get update && apt-get upgrade -y && apt-get install -y openssl libssl-dev pkg-config protobuf-compiler

# Path: /usr/src/app
RUN cargo install --path .

EXPOSE 3000

CMD ["tec-gtfs"]