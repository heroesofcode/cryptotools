#[cfg(test)]

use cryptotools::decode_base64::decode_base64;

#[test]
fn test_decode_base64() {
    let decode = decode_base64("MTIzNDU2Nzg5");
    let decode_string = String::from_utf8(decode).unwrap();
    assert_eq!(decode_string, "123456789");
}