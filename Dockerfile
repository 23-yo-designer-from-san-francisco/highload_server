FROM rust:latest

RUN apt update && apt install nginx -y

WORKDIR /usr/src/highload_server
COPY . .

RUN cargo build --release

CMD service nginx start && target/release/main
