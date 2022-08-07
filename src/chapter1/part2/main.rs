pub fn xor(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    assert_eq!(bytes1.len(), bytes2.len());
    bytes1.iter().zip(bytes2.iter()).map(|(b1, b2)| b1 ^ b2 ).collect()
}
fn main() {
    let b1 = hex::decode("1c0111001f010100061a024b53535009181c").expect("Decode fail");
    let b2 = hex::decode("686974207468652062756c6c277320657965").expect("Decode fail");
    let result = xor(&b1, &b2);
    println!("{}", hex::encode(result));

}