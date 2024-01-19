FROM rust:1.67

WORKDIR /usr/src/Rust_Connect_4
COPY . .

RUN cargo install --path .

