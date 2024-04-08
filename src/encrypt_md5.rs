pub fn encrypt_md5(data: &str) -> String {
    let digest = md5::compute(data);
    let digest_to_string = format!("{:02x?}", digest);
    return digest_to_string
}