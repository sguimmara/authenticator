# Authenticator

`Authenticator` is a simple rust project to study information security and authentication principles.

## Getting started

Clone this repository:
```shell
$ git clone git@github.com:sguimmara/authenticator.git
```

Build it using `cargo`:

```
$ cargo build
$ cargo run -- <subcommand> [ARGS]
```

Alternatively, you can build it using Docker :

```
$ docker build -t authenticator:example .
$ docker run authenticator:example <subcommand> [ARGS]
```

> Note: the Docker image builds the binary against the musl libc (see https://musl.libc.org/)

## Usage

`authenticator` is a CLI program, executed in the terminal. The API is the following form :

```shell
$ authenticator <subcommand> [ARGS]
```

### `hash` subcommand

This command computes then returns the SHA-256 hash of the supplied text.

```shell
$ authenticator hash "hello, world!"
SHA-256 of "hello, world!": 68e656b251e67e8358bef8483ab0d51c6619f3e7a1a9f0e75838d41ff368f728
```