pub fn reverse_bits(x: u32) -> u32 {
    // return x.reverse_bits();
    let mut x = x;
    let mut reversed = 0;
    for _ in 0..32 {
        reversed <<= 1;
        if x & 1 == 1 {
            reversed ^= 1;
        }
        x >>= 1;
    }

    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(reverse_bits(0b00000010100101000001111010011100), 0b00111001011110000010100101000000);
    }
}