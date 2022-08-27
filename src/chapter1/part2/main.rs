use common::xor::xor;

fn main() {
    let b1 = hex::decode("1c0111001f010100061a024b53535009181c").expect("Decode fail");
    let b2 = hex::decode("686974207468652062756c6c277320657965").expect("Decode fail");
    let result = xor(&b1, &b2);
    println!("{}", hex::encode(result));
}
