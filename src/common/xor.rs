/// XOR routine on bytes

/// Decrypt ciphertext with key using xor algorithm
/// both slices must be same length
pub fn xor_decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    assert_eq!(ciphertext.len(), key.len());
    ciphertext
        .iter()
        .zip(key.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect()
}

/// Decrypts message with single byte key
pub fn xor_decrypt_with_single_byte(ciphertext: &[u8], key: u8) -> Vec<u8> {
    let key_repeated = vec![key; ciphertext.len()];
    xor_decrypt(ciphertext, &key_repeated)
}

/// XORs message with repeating key
pub fn xor_decrypt_with_repeating_key(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let key_len = key.len();
    let mut result = Vec::new();
    for chunk in ciphertext.chunks(key_len) {
        let mut xored_chunk: Vec<u8> = chunk.iter().zip(key.iter()).map(|(c, z)| c ^ z).collect();
        result.append(&mut xored_chunk);
    }
    result
}
