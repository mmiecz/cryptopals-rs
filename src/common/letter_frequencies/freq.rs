use std::collections::HashMap;

///Struct holding ascii letters frequencies in given text
pub struct LetterFrequencies {
    text_len: usize,
    occurrences: HashMap<char, i32>,
}

impl LetterFrequencies {
    ///Loads LetterFrequencies with text input
    pub fn with_text(text: &str) -> LetterFrequencies {
        assert!(text.is_ascii());
        let mut occurrences = HashMap::new();
        let text_len = text.len();
        for letter in text.chars() {
            //NOT a letter actually, but...
            if letter.is_alphanumeric() {
                *occurrences.entry(letter).or_insert(0) += 1;
            }
        }

        LetterFrequencies {
            text_len,
            occurrences,
        }
    }

    pub fn get_count(&self, ch: char) -> i32 {
        *self.occurrences.get(&ch).unwrap_or(&0)
    }

    ///Count letter frequency in text (how often this letter was present) as decimal percentage
    pub fn get_frequency(&self, ch: char) -> i32 {
        *self.occurrences.get(&ch).unwrap_or(&0) * 100 / (self.text_len as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::letter_frequencies::freq::LetterFrequencies;

    #[test]
    fn test_letter_occurrence_is_counted_correctly() {
        let occ = LetterFrequencies::with_text("aaaaa");
        let a_count = occ.get_count('a');
        let b_count = occ.get_count('b');
        assert_eq!(a_count, 5);
        assert_eq!(b_count, 0);
    }

    #[test]
    fn test_letter_frequencies() {
        let test = "aaabbbcccd";
        let freqs = LetterFrequencies::with_text(test);
        let a_freq = freqs.get_frequency('a');
        assert_eq!(a_freq, 30);
        let d_freq = freqs.get_frequency('d');
        assert_eq!(d_freq, 10);
        let e_freq = freqs.get_frequency('e');
        assert_eq!(e_freq, 0);
    }
}
