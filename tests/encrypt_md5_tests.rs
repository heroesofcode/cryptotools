#[cfg(test)]

use cryptotools::encrypt_md5::*;

#[test]
fn test_encrypt_md5() {
    let md5 = MD5::encrypt_md5("9999");
    assert_eq!(md5, "fa246d0262c3925617b0c72bb20eeb1d");
}