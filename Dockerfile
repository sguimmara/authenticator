## Builder image
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

WORKDIR /src

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

## Final image
FROM scratch

WORKDIR /src

COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/authenticator ./

ENTRYPOINT ["./authenticator"]