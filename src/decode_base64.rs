pub fn decode_base64(data: &str) -> Vec<u8> {
    let base64_chars: Vec<char> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().collect();
    let mut result = Vec::new();
    let mut bytes = Vec::with_capacity(4);
    let mut padding_count = 0;

    count_number_chars(data, &mut padding_count, base64_chars, &mut bytes);
    remove_padding(&mut bytes);
    decode_bytes(&mut result, &bytes);
    remove_padding_from_result(&mut result, padding_count);

    return result;
}

fn count_number_chars(data: &str, padding_count: &mut usize, base64_chars: Vec<char>, bytes: &mut Vec<u8>) {
    for ch in data.chars() {
        if ch == '=' {
            *padding_count += 1;
        } else {
            let index = base64_chars.iter().position(|&c| c == ch).unwrap();
            bytes.push(index as u8);
        }
    }
}

fn remove_padding(bytes: &mut Vec<u8>) {
    while bytes.len() % 4 != 0 {
        bytes.pop();
    }
}

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

fn remove_padding_from_result(result: &mut Vec<u8>, padding_count: usize) {
    result.truncate(result.len() - padding_count);
}