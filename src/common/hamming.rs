fn hamming_dist_byte(val1: u8, val2: u8) -> u32 {
    let xored = val1 ^ val2;
    xored.count_ones()
}

pub fn hamming(val1: &[u8], val2: &[u8]) -> u32 {
    assert_eq!(val1.len(), val2.len());
    val1.iter()
        .zip(val2.iter())
        .fold(0, |acc, (v1, v2)| acc + hamming_dist_byte(*v1, *v2))
}

#[cfg(test)]
mod tests {
    use crate::hamming::hamming;

    #[test]
    fn proper_hamming_distance_on_string() {
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";
        let dist = hamming(&s1.as_bytes(), &s2.as_bytes());
        assert_eq!(dist, 37);
    }
}
