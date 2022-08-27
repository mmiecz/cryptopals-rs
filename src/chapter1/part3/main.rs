use common::english_detector::EnglishDetector;
use common::xor;
use common::xor::xor;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct ScoredKey {
    score: u32,
    decrypted_msg: Vec<u8>,
    key: u8,
}

impl PartialOrd for ScoredKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for ScoredKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

fn main() {
    let encoded = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let encrypted = hex::decode(encoded).expect("Decode error");
    let mut score_heap = BinaryHeap::new();
    let english_detector = EnglishDetector::init("assets/words.txt").expect("Can't open dict file");
    for key in 0..=255 {
        let key_decrypted = xor::xor_single_byte(&encrypted, key);
        let score = english_detector.detect_english(&key_decrypted);
        score_heap.push(ScoredKey {
            score,
            decrypted_msg: key_decrypted.clone(),
            key,
        })
    }

    println!("----5 best results------");
    for item in score_heap.iter().take(5) {
        println!(
            "Decrypted text: {}, Key: {:#x}, Score: {}",
            String::from_utf8_lossy(&item.decrypted_msg).to_string(),
            item.key,
            item.score
        );
    }
}
