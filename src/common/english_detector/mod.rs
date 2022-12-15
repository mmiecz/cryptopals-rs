use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::path::Path;

static ENGLISH_LETTERS_FREQ: Lazy<HashMap<u8, u32>> = Lazy::new(|| {
    [
        (b' ', 12100),
        (b'e', 12000),
        (b't', 9000),
        (b'a', 8000),
        (b'i', 8000),
        (b'o', 8000),
        (b'n', 8000),
        (b's', 8000),
        (b'h', 6400),
        (b'r', 6200),
        (b'd', 4400),
        (b'l', 4000),
        (b'u', 3400),
        (b'c', 3000),
        (b'm', 3000),
        (b'f', 2500),
        (b'w', 2000),
        (b'y', 2000),
        (b'g', 1700),
        (b'p', 1700),
        (b'b', 1600),
        (b'v', 1200),
        (b'k', 800),
        (b'q', 500),
        (b'j', 400),
        (b'x', 400),
        (b'z', 200),
    ]
    .iter()
    .cloned()
    .collect()
});
///Simple language detector based on dictionary
pub struct EnglishDetector {
    dict: HashSet<String>,
}

impl EnglishDetector {
    pub fn init<P: AsRef<Path>>(
        dict_path: P,
    ) -> Result<EnglishDetector, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(dict_path)?;
        let mut dict = HashSet::with_capacity(400_000);
        let lines = std::io::BufReader::new(file).lines();
        for word in lines {
            dict.insert(word?);
        }

        Ok(EnglishDetector { dict })
    }

    /// Detect english language and return  value from 0-100 describing confidence
    pub fn detect_english(&self, bytes: &[u8]) -> u32 {
        let v = Vec::from(bytes);
        match String::from_utf8(v) {
            Err(_) => 0,
            Ok(text) => {
                let words = text.split_whitespace();
                let mut words_total = 0;
                let mut words_english = 0;
                //Ignore non-ascii letters
                'word: for word in words {
                    //Only printable chars interests us.
                    for ch in word.chars() {
                        if !ch.is_alphanumeric() {
                            continue 'word;
                        }
                    }
                    words_total += 1;
                    if word.len() > 1 && self.dict.contains(word) {
                        words_english += 1;
                        println!("DEBUG DICT english word detected?: {}", word);
                    }
                }
                if words_total > 0 {
                    words_english * 100 / words_total
                } else {
                    0
                }
            }
        }
    }

    pub fn calculate_english_letter_score(ch: u8) -> u32 {
        if !ch.is_ascii() {
            return 0;
        }
        if ch.is_ascii_digit() {
            return 100;
        }
        let ch = ch.to_ascii_lowercase();
        let letter_scores = &ENGLISH_LETTERS_FREQ;
        match letter_scores.get(&ch) {
            Some(score) => *score,
            None => 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::english_detector::EnglishDetector;

    #[test]
    fn english_sentence_is_100_confident() {
        let english_detector =
            EnglishDetector::init("assets/words.txt").expect("Can't load dict file");

        let text = "To be or not to be";
        let confidence = english_detector.detect_english(text.as_bytes());
        assert_eq!(confidence, 100);
    }
}
