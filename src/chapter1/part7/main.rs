use common::aes::ecb_aes128;
fn main() {
    let input = include_str!("../../../assets/7.txt");
    let mut stripped = String::from(input).replace("\n", "");
    let decoded = base64::decode(stripped.as_bytes()).expect("Failed to base64 decode");

    let key = <[u8; 16]>::try_from("YELLOW SUBMARINE".as_bytes()).unwrap();
    let mut output = Vec::new();
    for block in decoded.chunks(16) {
        // TODO: Make this more ergonomic. Make conversions inside aes functions
        let block_16_bytes = <[u8; 16]>::try_from(block).unwrap();
        let decrypted_block = ecb_aes128::decrypt_block(block_16_bytes, key);
        output.extend_from_slice(&decrypted_block)
    }
    println!("{}", String::from_utf8_lossy(&output))
}
