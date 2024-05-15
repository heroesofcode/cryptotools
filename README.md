# cryptotools

[![CI](https://github.com/heroesofcode/cryptotools/actions/workflows/CI.yml/badge.svg)](https://github.com/heroesofcode/cryptotools/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/cryptotools)](https://crates.io/crates/cryptotools)
![Msrv](https://img.shields.io/badge/msrv-1.56.1-blue.svg?logo=rust&logoColor=orange)
[![Downloads](https://img.shields.io/crates/d/cryptotools.svg?logo=rust&logoColor=orange)](https://crates.io/crates/cryptotools)
[![Docs](https://docs.rs/cryptotools/badge.svg)](https://docs.rs/cryptotools)
[![License](https://img.shields.io/github/license/heroesofcode/cryptotools.svg)](https://github.com/heroesofcode/cryptotools/blob/main/LICENSE)

cryptotools is a cryptography library, with it you can:

- [x] Encode to base64
- [x] Decode the base64 value
- [x] Encrypt to md5

## Installing

In the file `Cargo.toml`

```toml
[dependencies]
cryptotools = "0.1.0"
```

## Usage

In the first example, if you want to encode and decode base64

```rust
use cryptotools::encode_base64::Base64Encode;
use cryptotools::decode_base64::Base64Decode;

// Encode
let encode = Base64Encode::encode("123456789");
println!("{}", encode);

// Decode
let decode = Base64Decode::decode("MTIzNDU2Nzg5");
println!("{}", decode);
```

To encrypt a value to md5

```rust
use cryptotools::encrypt_md5::MD5;

let md5 = MD5::encrypt("9999");
println!("{}", md5);
```

## Contributing

To contribute, just fork this project and then open a pull request, feel free to contribute, bring ideas and raise any problem in the issue tab.

## License

cryptotools is released under the MIT license. See [LICENSE](https://github.com/heroesofcode/cryptotools/blob/main/LICENSE) for details.
