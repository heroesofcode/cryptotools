pub fn encode_base64(data: &str) -> String {
    let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let mut result = String::new();
    let mut index = 0;

    let data_bytes = data.as_bytes();

    while index < data_bytes.len() {
        let mut bytes = [0u8; 3];
        let mut base64_indexes = Vec::new();
        let mut padding_count = 0;

        for i in 0..3 {
            if index < data_bytes.len() {
                bytes[i] = data_bytes[index];
                index += 1;
            } else {
                padding_count += 1;
            }
        }

        base64_indexes.push((bytes[0] >> 2) as u8);
        base64_indexes.push((((bytes[0] & 0x03) << 4) | ((bytes[1] & 0xF0) >> 4)) as u8);
        base64_indexes.push((((bytes[1] & 0x0F) << 2) | ((bytes[2] & 0xC0) >> 6)) as u8);
        base64_indexes.push((bytes[2] & 0x3F) as u8);

        for i in 0..(4 - padding_count) {
            result.push(base64_chars[base64_indexes[i] as usize] as char);
        }

        for _ in 0..padding_count {
            result.push('=');
        }
    }

    result
}