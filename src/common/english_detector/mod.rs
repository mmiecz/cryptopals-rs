use std::collections::HashSet;
use std::io::BufRead;
use std::path::Path;

///Simple language detector based on dictionary
pub struct EnglishDetector {
    dict: HashSet<String>,
}

struct TextDetectorStats {
    words_total: u32,
    known_words: u32,
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
}

#[cfg(test)]
mod tests {
    use crate::english_detector::EnglishDetector;

    #[test]
    fn english_sentence_is_100_confident() {
        let english_detector =
            EnglishDetector::init("assets/words.txt").expect("Can't load dict file");

        let text = "To be or not to be";
        let confidence = english_detector.detect_english(&text.as_bytes());
        assert_eq!(confidence, 100);
    }
}
