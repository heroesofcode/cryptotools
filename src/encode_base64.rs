/// Convert the value of the "data" parameter to base64 encode
/// Use: let encode = encode_base64("123456789");
pub fn encode_base64(data: &str) -> String {
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let mut result = String::new();
    let mut index = 0;

    let data_bytes = data.as_bytes();

    while index < data_bytes.len() {
        let mut bytes = [0u8; 3];
        let mut base64_indexes = Vec::new();
        let mut padding_count = 0;

        index = count_number_bytes(index, data_bytes, &mut bytes, &mut padding_count);
        calculate_base64_indexes(&bytes, &mut base64_indexes);
        append_base64_chars(&mut result, &base64_chars, &base64_indexes, padding_count);
    }

    result
}

fn count_number_bytes(mut index: usize, data_bytes: &[u8], bytes: &mut [u8; 3], padding_count: &mut usize) -> usize {
    for i in 0..3 {
        if index < data_bytes.len() {
            bytes[i] = data_bytes[index];
            index += 1;
        } else {
            *padding_count += 1;
        }
    }
    
    index
}

fn calculate_base64_indexes(bytes: &[u8; 3], base64_indexes: &mut Vec<u8>) {
    base64_indexes.push((bytes[0] >> 2) as u8);
    base64_indexes.push((((bytes[0] & 0x03) << 4) | ((bytes[1] & 0xF0) >> 4)) as u8);
    base64_indexes.push((((bytes[1] & 0x0F) << 2) | ((bytes[2] & 0xC0) >> 6)) as u8);
    base64_indexes.push((bytes[2] & 0x3F) as u8);
}

fn append_base64_chars(result: &mut String, base64_chars: &[u8], base64_indexes: &[u8], padding_count: usize) {
    for i in 0..(4 - padding_count) {
        result.push(base64_chars[base64_indexes[i] as usize] as char);
    }

    for _ in 0..padding_count {
        result.push('=');
    }
}