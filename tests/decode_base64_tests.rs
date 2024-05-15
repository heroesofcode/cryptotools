#[cfg(test)]

use cryptotools::decode_base64::*;

#[test]
fn test_decode_base64() {
    let decode = Base64Decode::decode_base64("MTIzNDU2Nzg5");
    assert_eq!(decode, "123456789");
}