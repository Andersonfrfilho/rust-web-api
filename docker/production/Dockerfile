# 1. This tells docker to use the Rust official image
FROM rust:1.70

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

CMD ["./target/release/rust-web-server"]