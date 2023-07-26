/// Don't use this in prod! ECB
mod ecb_aes128 {
    use aes::cipher::generic_array::GenericArray;
    use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
    use aes::Aes128;
    /// Decrypts the *input* with *key*
    /// It's allocating new buffer and returns it
    pub fn decrypt_block(input: [u8; 16], key: [u8; 16]) -> Vec<u8> {
        let key = GenericArray::from(key);
        let cipher = Aes128::new(&key);
        let mut block = GenericArray::from(input);
        cipher.decrypt_block(&mut block);
        block.as_slice().to_vec()
    }

    /// Encrypts the *input* with *key*
    /// It's allocating new buffer and returns it
    pub fn encrypt_block(input: [u8; 16], key: [u8; 16]) -> Vec<u8> {
        let cipher = Aes128::new(&GenericArray::from(key));
        let mut block = GenericArray::from(input);
        cipher.encrypt_block(&mut block);
        block.as_slice().to_vec()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn decrypt_aes128_ecb() {
            let key = <[u8; 16]>::try_from("secret_key_16bit".as_bytes()).unwrap();
            let input_encrypt = <[u8; 16]>::try_from("secret_msg_16bit".as_bytes()).unwrap();
            let encrypted_block = encrypt_block(input_encrypt, key);

            let input_decrypt = <[u8; 16]>::try_from(encrypted_block.as_slice()).unwrap();
            let decrypted = decrypt_block(input_decrypt, key);
            assert_eq!("secret_msg_16bit", String::from_utf8_lossy(&decrypted));
        }
    }
}
