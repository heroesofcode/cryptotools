use cryptotools::encode_base64::Base64Encode;
use cryptotools::decode_base64::Base64Decode;
use cryptotools::encrypt_md5::MD5;

fn main() {
    // Encode
    let encode = Base64Encode::enconde("123456789");
    println!("{}", encode);

    // Decode
    let decode = Base64Decode::decode("MTIzNDU2Nzg5");
    println!("{}", decode);

    // md5
    let md5 = MD5::encrypt("9999");
    println!("{}", md5);
}
