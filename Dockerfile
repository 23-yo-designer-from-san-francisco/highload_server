FROM rust:latest

WORKDIR /usr/src/highload_server
COPY . .

RUN cargo build --release

CMD ["target/release/main"]
