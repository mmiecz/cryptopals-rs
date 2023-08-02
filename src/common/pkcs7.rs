/// PKCS Padding

/// Padding up to 256
pub fn pad(input: &[u8], pad: u8) -> Vec<u8> {
    let remainder = input.len() % pad as usize;
    println!("{remainder}");
    let padding = if remainder == 0 {
        pad
    } else {
        pad - remainder as u8
    };
    let pad = vec![padding; padding as usize];
    let mut res = Vec::from(input);
    res.extend_from_slice(&pad);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_padding() {
        let input = [0x1; 14];
        let padded = pad(&input, 16);
        assert_eq!(padded, [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2]);

        let input = [0x1; 2];
        let padded = pad(&input, 10);
        assert_eq!(padded, [1, 1, 8, 8, 8, 8, 8, 8, 8, 8]);

        let input = [0x1; 4];
        let padded = pad(&input, 4);
        assert_eq!(padded, [1, 1, 1, 1, 4, 4, 4, 4]);
    }
}
