---
layout: default
title: VeTiS - Very Tiny Server
nav_order: 1
description: "🚀 A blazingly fast, minimalist HTTP server built for modern Rust applications"
permalink: /
---
<div align="center">
<h1><b>VeTiS</b></h1>
</div>

[![crates.io](https://img.shields.io/crates/v/vetis?style=flat-square)](https://crates.io/crates/vetis)
[![Build Status](https://github.com/ararog/vetis/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/vetis/actions/workflows/rust.yml)
[![Documentation](https://docs.rs/vetis/badge.svg)](https://docs.rs/vetis/latest/vetis)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**VeTiS** is a lightweight yet powerful web server that brings simplicity and performance together. Designed with Rust's safety guarantees in mind, it delivers HTTP/1, HTTP/2, and HTTP/3 support with a clean, intuitive API that makes building web services a breeze.

Built on top of [hyper](https://github.com/hyperium/hyper).

## Features

- **🎯 Minimalist Design**: Focus on what matters - serving HTTP requests efficiently
- **🔧 Flexible Runtime**: Choose between Tokio or Smol async runtimes
- **🌐 Protocol Support**: Full HTTP/1, HTTP/2, and HTTP/3 implementation
- **🛡️ Secure by Default**: Built-in TLS support with modern cryptography
- **⚡ Zero-Cost Abstractions**: Leverage Rust's performance without overhead
- **📦 Feature-Gated**: Include only what you need for optimal binary size

## 🛠️ Quick Start

Add VeTiS to your `Cargo.toml`:

```rust
vetis = { version = "0.1.0", features = ["tokio-rt", "http2", "tokio-rust-tls"] }
```

Basic usage:

```rust
use bytes::Bytes;
use clap::Parser;
use http_body_util::Full;
use hyper::Response;
use vetis::{server::config::ServerConfig, Vetis};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short = 'p',
        long,
        required = false,
        num_args = 0..=1,
        require_equals = true,
        default_value = "8080",
        help = "Port to listen on"
    )]
    port: u16,

    #[arg(
        short = 'i',
        long,
        required = false,
        num_args = 0..=1,
        require_equals = true,
        default_value = "0.0.0.0",
        help = "Interface to bind to"
    )]
    interface: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = ServerConfig::builder()
        .port(args.port)
        .interface(args.interface)
        .build();

    let mut server = Vetis::new(config);

    server
        .run(|_| async move { 
            Ok(Response::new(Full::new(Bytes::from("Hello from VeTiS! 🚀")))) 
        })
        .await?;

    server.stop().await?;

    Ok(())
}
```

## Examples

Check out the [examples](./examples.md) for complete examples of how to use Vetis in your projects.

## Create project from template

You can create a new project from the template using `cargo generate`:

`cargo generate ararog/vetis-templates`

## Documentation

- [API Reference](https://docs.rs/vetis)
- [Contributing Guide](./CONTRIBUTING.md)

## License

This project is licensed under the [MIT License](./LICENSE.md).

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
