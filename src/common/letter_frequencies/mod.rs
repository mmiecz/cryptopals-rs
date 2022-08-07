use std::collections::HashMap;

///Struct holding ascii letters frequencies in given text
pub struct LetterFrequencies {
    text_len: usize,
    occurences: HashMap<char, i32>
}

impl LetterFrequencies {
    ///Loads LetterFrequencies with text input
    pub fn with_text(text: &str) -> LetterFrequencies {
        assert!(text.is_ascii());
        let mut occurrences = HashMap::new();
        let text_len = text.len();
        for letter in text.chars() {
            *occurences.entry(letter).or_insert(0) += 1;
        }

        LetterFrequencies {
            text_len,
            occurences
        }
    }

    pub fn get_count(&self, ch: char) -> i32 {
        *self.occurences.get(&ch).unwrap_or(&0)
    }

    ///Count letter frequency in text (how often this letter was present)
    pub fn get_frequency(&self, ch: char) -> f32 {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use crate::letter_frequencies::LetterFrequencies;

    #[test]
    fn test_letter_occurrence_is_counted_correctly() {
        let occ = LetterFrequencies::with_text("aaaaa");
        let a_count = occ.get_count('a');
        let b_count = occ.get_count('b');
        assert_eq!(a_count, 5);
        assert_eq!(b_count, 0);
    }
}