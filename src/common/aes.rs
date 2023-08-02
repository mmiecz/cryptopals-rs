/// Don't use this in prod! ECB
/// Key size is 128
///
pub const AES_BLOCK_SIZE: usize = 16;
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
            let key = "secret_key_16bit".as_bytes().try_into().unwrap();
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

pub mod cbc_aes128 {
    use super::ecb_aes128;
    use crate::xor::xor_decrypt;
    /// To decrypt CBC
    /// Take IV and the key
    /// To decrypt the block, apply ECB decrypt with the key,
    /// and xor the result with previous ENCRYPTED block
    /// First block is XORed with IV
    pub struct Aes128CBC {
        key: [u8; 16],
        /// previous block, or in case of initial block decryption, IV
        previous_block: [u8; 16],
    }

    impl Aes128CBC {
        pub fn init(key: [u8; 16], iv: [u8; 16]) -> Aes128CBC {
            Aes128CBC {
                key,
                previous_block: iv,
            }
        }

        /// Give next block to decipher.
        /// Updates internal state, and returns deciphered block
        pub fn feed_block(&mut self, block: [u8; 16]) -> [u8; 16] {
            let block_decrypted = ecb_aes128::decrypt_block(block, self.key);
            let xored = xor_decrypt(&block_decrypted, &self.previous_block);
            self.previous_block = block;
            xored.try_into().unwrap()
        }
    }
}
