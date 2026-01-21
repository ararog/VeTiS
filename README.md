# VeTiS (Very Tiny Server)

Very tiny server.

## Description

**Vetis** is a small webserver.

## Install

```rust
vetis = { version = "0.1.0", features = ["tokio-rt", "http2", "tokio-rust-tls"] }
```

## Runtimes

- [tokio](https://github.com/tokio-rs/tokio)
- [smol](https://github.com/smol-rs/smol)

## Crate features

- tokio-rt (default)
- smol-rt
- http1
- http2 (default)
- http3
- tokio-rust-tls (default)


## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>

