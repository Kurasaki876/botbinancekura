FROM rust:1.72

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
    cargo build --release

CMD ["./target/release/ia-strategic-core"]
