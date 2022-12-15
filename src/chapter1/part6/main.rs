use base64;
use common::{
    english_detector::EnglishDetector,
    hamming::hamming,
    xor::{xor_repeating_key, xor_single_byte},
};
use log::{debug, info};

// Returns 5 best most promising keysizes
fn find_best_key_sizes(input: &[u8]) -> Vec<usize> {
    let mut key_scores = Vec::new();
    for key_size in 2..=40 {
        if let Some(score) = get_keysize_hamming_score(&input, key_size) {
            key_scores.push((score, key_size));
        }
    }
    key_scores.sort_by(|(s1, _), (s2, _)| s1.partial_cmp(s2).unwrap());
    debug!("{:?}", key_scores);
    key_scores.iter().map(|(_, len)| *len).take(5).collect()
}

// Calculate score of single char.
// More popular english letters (e, t, a o ) have higher score than the lower ones
// Non-ascii char has 0 score
fn letter_score(letter: u8) -> u32 {
    static COMMON_ENGLISH_LETTERS: &str = " ETAONRISHDLFCMUGYPWBVKJXZQ";
    if let Some(position) = COMMON_ENGLISH_LETTERS.find(letter.to_ascii_uppercase() as char) {
        100 - position as u32 * 2 // letters that are more common have higher score
    } else {
        if letter.is_ascii_alphanumeric() {
            10
        } else {
            0
        }
    }
}

// returns normalized hamming distance score for given key_size.
fn get_keysize_hamming_score(text: &[u8], key_size: usize) -> Option<f32> {
    let chunk_size = key_size * 2;
    info!("testing key size: {key_size}, with chunk_size: {chunk_size}");
    if chunk_size > text.len() / 2 {
        info!("Chunk size {chunk_size} is bigger than half input text, skipping");
        return None;
    }
    let mut score = 0.0;
    let mut measures = 0;
    for big_chunk in text.chunks(chunk_size * 2) {
        if big_chunk.len() < chunk_size * 2 {
            break;
        }
        let left_chunk = &big_chunk[0..chunk_size];
        let right_chunk_end = chunk_size * 2;
        let right_chunk = &big_chunk[chunk_size..right_chunk_end];
        let hamming_distance = hamming(&left_chunk, &right_chunk);
        score += hamming_distance as f32;
        measures += 1;
    }
    score /= key_size as f32;
    score /= measures as f32;
    Some(score)
}

// This iterates over all possible keys and find best matching
// one ( that is, it decodes to most common english letters and ascii in general )
fn find_best_single_xor_key(input: &[u8], key_len: usize) -> u8 {
    let sample_len = input.len();
    if sample_len == 0 {
        return 0;
    }
    let mut max_score_byte: (f32, u8) = (0.0, 0);
    for k in 0..255 {
        let decoded_bytes = xor_single_byte(input, k); // decodes input string, result will be used to determine xor key score
        let mut score = 0.0;
        for byte in decoded_bytes.iter() {
            score += EnglishDetector::calculate_english_letter_score(*byte) as f32;
        }
        //normalize it against sample len, and key len
        score /= sample_len as f32;
        score /= key_len as f32;
        if score.gt(&max_score_byte.0) {
            max_score_byte = (score, k);
        }
    }
    max_score_byte.1
}
// transpose...
fn transpose_ciphertext_from(input: &[u8], key_len: usize, from: usize) -> Vec<u8> {
    let slice = &input[from..];
    let mut transposed_line: Vec<u8> = Vec::new();
    for b in slice.iter().step_by(key_len) {
        transposed_line.push(*b);
    }
    transposed_line
}

// For given key length, find most promising (i.e. having most english letters key, since key is in english )
// The first letter of the key will encrypt first byte every key_len chunk, second letter second byte, etc.
fn find_key(input: &[u8], key_len: usize) -> Vec<u8> {
    //gather transpositions and find best key match for each
    let mut key = Vec::new();
    for i in 0..key_len {
        let transposed = transpose_ciphertext_from(input, key_len, i);
        let key_elem = find_best_single_xor_key(&transposed, key_len);
        key.push(key_elem)
    }
    key
}
fn main() {
    env_logger::init();
    let s = include_str!("../../../assets/input1-6.txt");
    let input = base64::decode(s.as_bytes()).expect("base64 decoding error");

    debug!("Loaded {} bytes", input.len());
    let key_sizes = find_best_key_sizes(&input);
    debug!("Found best key sizes: {:?}", key_sizes);
    let encryption_key = find_key(&input, key_sizes[0]);
    let key = String::from_utf8_lossy(&encryption_key);
    let decrypted_bytes = xor_repeating_key(&input, &encryption_key);
    let message_decrypted = String::from_utf8_lossy(&decrypted_bytes);
    println!("found: {key}");
    println!("decrypted message: {message_decrypted}");
}
