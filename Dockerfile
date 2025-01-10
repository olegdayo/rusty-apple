FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM --platform=linux/amd64 debian:latest AS runner
WORKDIR /app
RUN sudo apt update
RUN sudo apt -y install ffmpeg
COPY --from=builder /app/target/release/rusty-apple .
CMD ["./rusty-apple"]
