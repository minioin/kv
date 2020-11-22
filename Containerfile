FROM ekidd/rust-musl-builder:latest AS builder
WORKDIR /workdir
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM alpine
EXPOSE 8000
COPY --from=builder /workdir/target/x86_64-unknown-linux-musl/release/kv /usr/local/bin/
CMD ["/usr/local/bin/kv"]