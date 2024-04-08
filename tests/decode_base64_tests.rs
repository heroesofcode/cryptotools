#[cfg(test)]

use cryptotools::decode_base64::decode_base64;

#[test]
fn test_decode_base64() {
    let decoded = decode_base64("MTIzNDU2Nzg5");
    let decoded_string = String::from_utf8(decoded).unwrap();
    assert_eq!(decoded_string, "123456789");
}