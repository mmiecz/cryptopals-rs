use common::english_detector::EnglishDetector;
use common::xor::xor_single_byte;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq)]
struct ScoredText {
    text: String,
    score: u32,
    key: u8,
}

impl PartialOrd for ScoredText {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for ScoredText {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

fn main() {
    let input = File::open("assets/input-1-4.txt").expect("Cant find file");
    let lines = BufReader::new(input).lines();
    let english_detector =
        EnglishDetector::init("assets/words.txt").expect("Failed to open dict file");
    let mut results = BinaryHeap::with_capacity(60);
    for line in lines {
        let line = line.unwrap();
        let hexed = hex::decode(&line).expect("Decode error");
        for key in 0..=255 {
            let decrypted = xor_single_byte(&hexed, key);
            let score = english_detector.detect_english(&decrypted);

            if score > 0 {
                results.push(ScoredText {
                    text: String::from_utf8_lossy(&decrypted).to_string(),
                    score,
                    key,
                })
            }
        }
    }

    for result in results.iter() {
        if result.score > 50 {
            println!(
                "score: {} key: {:#x} text: {}",
                result.score, result.key, result.text
            );
        }
    }
}
