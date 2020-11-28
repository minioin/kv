FROM rust AS builder
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo new kv
WORKDIR /usr/src/kv
COPY Cargo.toml Cargo.lock ./

# Cache build dependencies
RUN cargo build --release

# Finally build the executable
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/kv .
EXPOSE 8000
CMD ["./kv"]
