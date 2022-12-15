use common::xor::xor_decrypt_with_repeating_key;

fn main() {
    let msg = r#"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"#;
    let key = "ICE";
    let res = xor_decrypt_with_repeating_key(msg.as_bytes(), key.as_bytes());
    let hex = hex::encode(res);
    println!("{hex}");
    assert_eq!(
        hex,
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2\
        f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
            .to_string()
    );
}
