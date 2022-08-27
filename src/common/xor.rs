///XOR routine on bytes
pub fn xor(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    assert_eq!(bytes1.len(), bytes2.len());
    bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect()
}

pub fn xor_single_byte(msg: &[u8], key: u8) -> Vec<u8> {
    let key_repeated = vec![key; msg.len()];
    xor(&msg, &key_repeated)
}

/// XORs message with repeating key
pub fn xor_repeating_key(msg: &[u8], key: &[u8]) -> Vec<u8> {
    let key_len = key.len();
    let mut result = Vec::new();
    for chunk in msg.chunks(key_len) {
        let mut xored_chunk: Vec<u8> = chunk.iter().zip(key.iter()).map(|(c, z)| c ^ z).collect();
        result.append(&mut xored_chunk);
    }
    result
}
