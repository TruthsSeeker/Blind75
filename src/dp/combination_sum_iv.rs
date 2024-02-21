pub fn combination_sum_iv(nums: Vec<i32>, target: i32) -> i32 {
    let mut tab = vec![0; (target + 1) as usize];
    tab[0] = 1;
    for i in 1..tab.len() {
        for n in &nums {
            if i as i32 - n >= 0 {
                tab[i] += tab[i - *n as usize];
            }
        }
    }
    tab[target as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum_iv() {
        assert_eq!(combination_sum_iv(vec![1, 2, 3], 4), 7);
        assert_eq!(combination_sum_iv(vec![9], 3), 0);
    }
}