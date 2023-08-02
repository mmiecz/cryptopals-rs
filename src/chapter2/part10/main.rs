use common::aes::{cbc_aes128, AES_BLOCK_SIZE};
fn main() {
    let input = include_str!("../../../assets/input-2-10.txt");
    let stripped = String::from(input).replace("\n", "");
    let decoded = base64::decode(stripped.as_bytes()).expect("Failed to base64 decode");
    let iv = ['\x00' as u8; AES_BLOCK_SIZE];
    let key = "YELLOW SUBMARINE".as_bytes().try_into().unwrap();
    let mut cbc = cbc_aes128::Aes128CBC::init(key, iv);
    let mut decrypted = Vec::new();
    for block in decoded.chunks(AES_BLOCK_SIZE) {
        let res = cbc.feed_block(block.try_into().unwrap());
        decrypted.extend_from_slice(&res);
    }
    let result = String::from_utf8_lossy(&decrypted);
    println!("{result}");
}
