/// Provides methods to decode Base64 encoded strings.
///
/// # Example
/// ```
/// use cryptotools::decode_base64::Base64Decode;
/// let decoded = Base64Decode::decode("aGVsbG8=");
/// println!("{decoded}");
/// ```
pub struct Base64Decode;

impl Base64Decode {
	/// Decodes a Base64 string slice into a regular string.
	///
	/// # Arguments
	///
	/// * `value` - The Base64-encoded string slice to decode.
	///
	/// # Returns
	///
	/// A decoded string.
	pub fn decode(value: &str) -> String {
		let base64_chars: Vec<char> =
			"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
				.chars()
				.collect();
		let mut result = Vec::new();
		let mut bytes = Vec::with_capacity(4);
		let mut padding_count = 0;

		ValidationDecodeBase64::count_number_chars(value, &mut padding_count, base64_chars, &mut bytes);
		ValidationDecodeBase64::remove_padding(&mut bytes);
		ValidationDecodeBase64::decode_bytes(&mut result, &bytes);
		ValidationDecodeBase64::remove_padding_from_result(&mut result, padding_count);

		let decode_string = String::from_utf8(result).unwrap();
		decode_string
	}
}

/// Collection of helper methods for validating and processing the Base64 decoding.
struct ValidationDecodeBase64;

impl ValidationDecodeBase64 {
	/// Converts Base64 characters to indexes, tracks padding, and fills byte vector.
	fn count_number_chars(
		data: &str,
		padding_count: &mut usize,
		base64_chars: Vec<char>,
		bytes: &mut Vec<u8>,
	) {
		for ch in data.chars() {
			if ch == '=' {
				*padding_count += 1;
			} else {
				let index = base64_chars.iter().position(|&c| c == ch).unwrap();
				bytes.push(index as u8);
			}
		}
	}

	/// Removes any extra values so the number of bytes is a multiple of 4.
	fn remove_padding(bytes: &mut Vec<u8>) {
		while bytes.len() % 4 != 0 {
			bytes.pop();
		}
	}

	/// Decodes byte chunks from Base64 index representation into bytes.
	fn decode_bytes(result: &mut Vec<u8>, bytes: &[u8]) {
		for i in (0..bytes.len()).step_by(4) {
			let byte1 = (bytes[i] << 2) | (bytes[i + 1] >> 4);
			let byte2 = ((bytes[i + 1] & 0x0F) << 4) | (bytes[i + 2] >> 2);
			let byte3 = ((bytes[i + 2] & 0x03) << 6) | bytes[i + 3];
			result.push(byte1);
			result.push(byte2);
			result.push(byte3);
		}
	}

	/// Truncates the last bytes according to the padding count to get the final decoded result.
	fn remove_padding_from_result(result: &mut Vec<u8>, padding_count: usize) {
		result.truncate(result.len() - padding_count);
	}
}
