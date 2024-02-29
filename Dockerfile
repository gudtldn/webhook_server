FROM rust:1.76.0-slim

EXPOSE 7899/tcp

COPY . /data
WORKDIR /data

RUN cargo build --release

CMD ["./target/release/webhook_server"]
