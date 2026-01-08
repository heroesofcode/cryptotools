/// Provides methods to hash strings using the MD5 algorithm.
///
/// # Example
/// ```
/// use cryptotools::encrypt_md5::MD5;
/// let hash = MD5::encrypt("hello");
/// println!("{hash}");
/// ```
pub struct MD5;

impl MD5 {
	/// Hashes a string slice using MD5 and returns a hexadecimal representation.
	///
	/// # Arguments
	///
	/// * `value` - The string slice to hash.
	///
	/// # Returns
	///
	/// The MD5 hash as a hexadecimal string.
	pub fn encrypt(value: &str) -> String {
		let digest = md5::compute(value);
		let digest_to_string = format!("{:02x?}", digest);
		digest_to_string
	}
}
