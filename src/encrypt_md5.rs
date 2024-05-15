pub struct MD5;

impl MD5 {
    /// Function to convert string to md5
    pub fn encrypt(value: &str) -> String {
        let digest = md5::compute(value);
        let digest_to_string = format!("{:02x?}", digest);
        return digest_to_string
    }
}