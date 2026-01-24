# stegano-f5-jpeg-decoder

This is a fork of [image-rs/jpeg-decoder](https://github.com/image-rs/jpeg-decoder) with modifications for 
[stegano-f5](https://github.com/steganogram/stegano-rs/tree/main/crates/stegano-f5).

Which is a specific steganographic algorithm for jpeg images.

---

## original jpeg-decoder README

**WARNING: This crate is in maintenance mode as _image-rs_ seeks to move over to the zune-jpeg crate. However, since other Rust JPEG crates do not support lossless mode, we will continue to welcome PRs related to that functionality.**

[![Rust CI](https://github.com/steganogram/stegano-f5-jpeg-decoder/workflows/Rust%20CI/badge.svg)](https://github.com/steganogram/stegano-f5-jpeg-decoder/actions)
[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/k65rrkd0f8yb4o9w/branch/master?svg=true)](https://ci.appveyor.com/project/kaksmet/jpeg-decoder/branch/master)
[![crates.io](https://img.shields.io/crates/v/jpeg-decoder.svg)](https://crates.io/crates/jpeg-decoder)

A Rust library for decoding JPEGs.

[Documentation](https://docs.rs/jpeg-decoder)

## Example

Cargo.toml:
```toml
[dependencies]
jpeg-decoder = "0.3"
```

main.rs:
```rust
extern crate stegano_f5_jpeg_decoder as jpeg;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("hello_world.jpg").expect("failed to open file");
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
}
```

## Requirements

This crate compiles with Rust >= 1.61. Minimum supported Rust version:
- All releases `0.1.*` compile with Rust >= 1.36.
- All releases `0.2.*` compile with Rust >= 1.48.
