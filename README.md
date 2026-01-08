# cryptotools

[![CI](https://github.com/heroesofcode/cryptotools/actions/workflows/CI.yml/badge.svg)](https://github.com/heroesofcode/cryptotools/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/cryptotools)](https://crates.io/crates/cryptotools)
[![Docs](https://docs.rs/cryptotools/badge.svg)](https://docs.rs/cryptotools)
[![License](https://img.shields.io/github/license/heroesofcode/cryptotools.svg)](https://github.com/heroesofcode/cryptotools/blob/main/LICENSE)

cryptotools is a simple, easy-to-use library for cryptographic utilities in Rust. It currently provides the following:

- [x] Base64 encoding
- [x] Base64 decoding
- [x] MD5 encryption (hashing)

## Installing

cargo add cryptotools

```toml
[dependencies]
cryptotools = "0.3.0"
```

## Usage

### Base64 Encoding & Decoding

```rust
use cryptotools::encode_base64::Base64Encode;
use cryptotools::decode_base64::Base64Decode;

let input = "hello world";
let encoded = Base64Encode::encode(input);
println!("Base64 Encoded: {}", encoded);

let decoded = Base64Decode::decode(&encoded);
println!("Base64 Decoded: {}", decoded);
```

### MD5 Hashing

```rust
use cryptotools::encrypt_md5::MD5;

let input = "password123";
let hash = MD5::encrypt(input);
println!("MD5 Hash: {}", hash);
```

## License

cryptotools is released under the MIT license. See [LICENSE](https://github.com/heroesofcode/cryptotools/blob/main/LICENSE) for details.
