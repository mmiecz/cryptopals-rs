/// Don't use this in prod! ECB
/// Key size is 128
pub mod ecb_aes128 {
    use aes::cipher::generic_array::GenericArray;
    use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
    use aes::Aes128;
    use anyhow::anyhow;

    /// Decrypts the *input* with *key*
    /// It's allocating new buffer and returns it
    pub fn decrypt_block(input: [u8; 16], key: [u8; 16]) -> [u8; 16] {
        let key = GenericArray::from(key);
        let cipher = Aes128::new(&key);
        let mut block = GenericArray::from(input.clone());
        cipher.decrypt_block(&mut block);
        block.into()
    }

    /// Encrypts the *input* with *key*
    /// It's allocating new buffer and returns it
    pub fn encrypt_block(input: [u8; 16], key: [u8; 16]) -> [u8; 16] {
        let cipher = Aes128::new(&GenericArray::from(key));
        let mut block = GenericArray::from(input);
        cipher.encrypt_block(&mut block);
        block.into()
    }

    /// Decrypts slice with aes128 ecb.
    /// No padding or anything is added, so you must pass *input* exactly multiple of 16 bytes
    pub fn decrypt_slice(input: &[u8], key: [u8; 16]) -> anyhow::Result<Vec<u8>> {
        if input.len() % 16 != 0 {
            return Err(anyhow!("input not multiple of 16 bytes!"));
        }
        let blocks_num = input.len() / 16;
        let mut buffer = Vec::with_capacity(blocks_num);
        for block in input.chunks(16) {
            let decrypted_block = decrypt_block(block.try_into().unwrap(), key);
            buffer.extend_from_slice(&decrypted_block);
        }
        Ok(buffer)
    }

    /// Encryptd slice with aes128 ecb.
    /// No padding or anything is added, so you must pass *input* exactly multiple of 16 bytes
    pub fn encrypt_slice(input: &[u8], key: [u8; 16]) -> anyhow::Result<Vec<u8>> {
        if input.len() % 16 != 0 {
            return Err(anyhow!("input not multiple of 16 bytes!"));
        }
        let blocks_num = input.len() / 16;
        let mut buffer = Vec::with_capacity(blocks_num);
        for block in input.chunks(16) {
            let encrypted_block = encrypt_block(block.try_into().unwrap(), key);
            buffer.extend_from_slice(&encrypted_block);
        }
        Ok(buffer)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn encrypt_decrypt_aes128_block() {
            let key = <[u8; 16]>::try_from("secret_key_16bit".as_bytes()).unwrap();
            let input_encrypt = <[u8; 16]>::try_from("secret_msg_16bit".as_bytes()).unwrap();
            let encrypted_block = encrypt_block(input_encrypt, key);
            let input_decrypt = <[u8; 16]>::try_from(encrypted_block.as_slice()).unwrap();
            let decrypted = decrypt_block(input_decrypt, key);
            assert_eq!("secret_msg_16bit", String::from_utf8_lossy(&decrypted));
        }

        #[test]
        fn passing_slice_to_aes128_encrypt_slice_that_is_not_aligned_is_error() {
            let key = <[u8; 16]>::try_from("secret_key_16bit".as_bytes()).unwrap();
            let input_to_encrypt = "secret_msg_16bits".as_bytes();
            let result = encrypt_slice(input_to_encrypt, key);
            assert!(result.is_err())
        }

        #[test]
        fn ecrypt_decrypt_aes128_block() {
            let key = <[u8; 16]>::try_from("secret_key_16bit".as_bytes()).unwrap();
            let input_to_encrypt = "secret_msg_16bitsecret_msg_16bit".as_bytes();
            let encrypted = encrypt_slice(input_to_encrypt, key).unwrap();
            let decrypted = decrypt_slice(&encrypted, key).unwrap();
            assert_eq!(
                "secret_msg_16bitsecret_msg_16bit",
                String::from_utf8_lossy(&decrypted)
            );
        }
    }
}
