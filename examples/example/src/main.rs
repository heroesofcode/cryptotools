use cryptotools::encode_base64::encode_base64;
use cryptotools::decode_base64::decode_base64;
use cryptotools::encrypt_md5::encrypt_md5;

fn main() {
    // Encode
    let encode = encode_base64("123456789");
    println!("{}", encode);

    // Decode
    let decode = decode_base64("MTIzNDU2Nzg5");
    let decode_string = String::from_utf8(decode).unwrap();
    println!("{}", decode_string);

    // md5
    let md5 = encrypt_md5("9999");
    println!("{}", md5);
}
