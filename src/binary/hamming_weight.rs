pub fn hamming_weight (n: u32) -> i32 {
    let mut count = 0;
    let mut m = n;
    while m != 0 {
        count += m & 1;
        m = m >> 1;
    }
    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(hamming_weight(0b00000000000000000000000000001011), 3);
        assert_eq!(hamming_weight(0b00000000000000000000000010000000), 1);
    }
}