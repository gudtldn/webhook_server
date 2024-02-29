FROM rust:1.76.0-slim

EXPOSE 7899/tcp

COPY . /data

CMD ["./target/release/webhook_server"]
