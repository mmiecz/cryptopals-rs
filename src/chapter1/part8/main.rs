use common::aes::ecb_aes128;
use std::cmp::Ordering;
use std::collections::HashSet;

fn has_duplicate_block(input: &[u8]) -> bool {
    let mut chunks_seen = HashSet::new();
    let chunks = input.chunks(16);
    for chunk in chunks {
        if chunks_seen.contains(chunk) {
            return true;
        }
        chunks_seen.insert(chunk);
    }
    false
}

fn main() {
    let input = include_str!("../../../assets/input-1-8.txt");
    /// We're trying to find the same chunk in the line and hope for the best
    for (i, line) in input.lines().enumerate() {
        let unhexed = hex::decode(line).unwrap();
        if has_duplicate_block(&unhexed) {
            println!("{i}: {line}");
        }
    }
}
