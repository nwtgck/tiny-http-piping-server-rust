# tiny-http-piping-server
[![CircleCI](https://circleci.com/gh/nwtgck/tiny-http-piping-server-rust.svg?style=shield)](https://circleci.com/gh/nwtgck/tiny-http-piping-server-rust)

Piping Server written in Rust ([tiny-http](https://github.com/tiny-http/tiny-http))

## Purpose
**Faster Piping Server than ever**  
This has the same purpose as <https://github.com/nwtgck/piping-server-rust>.

* Faster is better
* Low memory cost
* Machine-friendly implementation

## Why Rust?
Safe, Fast and No garbage collection (GC)

## Why tiny-http?
The project above uses [Hyper](https://github.com/hyperium/hyper). However, tiny-http is easier to write low level HTTP server.

## Run a server
You can choose Cargo or Docker to run a server.

### Cargo
```rs
cargo run --release
```

### Docker
Run a Piping server on <http://localhost:8181> by the following command.

```rs
 docker run -p 8181:8080 --init nwtgck/tiny-http-piping-server-rust
```

### Server-side help

```txt
Piping Server in Rust (tiny-http)

USAGE:
    tiny-http-piping-server [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --http-port <http-port>    Image width [default: 8080]

```
