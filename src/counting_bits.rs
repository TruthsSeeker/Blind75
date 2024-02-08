pub fn count_bits(n: i32) -> Vec<i32> {
    let mut result = vec![0];
    for i in 1..=n {
        let count =  (i % 2) + result[(i/2) as usize];
        result.push(count);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
        assert_eq!(count_bits(8), vec![0, 1, 1, 2, 1, 2, 2, 3, 1]);
        assert_eq!(count_bits(10), vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2]);
    }
}