# Authenticator

`Authenticator` is a simple rust project to study information security and authentication principles.

## Getting started

Clone this repository:
```shell
git clone git@github.com:sguimmara/authenticator.git
```

Build it using `cargo`:

```
cargo build
cargo run -- <subcommand> [ARGS]
```

Alternatively, you can build it using Docker :

```
docker build -t authenticator:example .
docker run authenticator:example <subcommand> [ARGS]
```

> Note: the Docker image builds the binary against the musl libc (see https://musl.libc.org/)