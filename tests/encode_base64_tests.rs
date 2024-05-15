#[cfg(test)]

use cryptotools::encode_base64::*;

#[test]
fn test_encode_base64() {
    let encode = Base64Encode::enconde("123456789");
    assert_eq!(encode, "MTIzNDU2Nzg5");
}