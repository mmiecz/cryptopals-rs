fn main() {
    let input = include_str!("../../../assets/7.txt");
    let decoded = base64::decode(input).expect("Failed to base64 decode");
    let key = "YELLOW SUBMARINE".as_bytes();
}
