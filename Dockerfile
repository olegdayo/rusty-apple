FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN apt update && apt install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM --platform=linux/amd64 alpine:latest AS runner
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rusty-apple .
CMD ["./rusty-apple"]
