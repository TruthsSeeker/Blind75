pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut dp = vec![vec![1; n as usize]; m as usize];

    for x in 1..m as usize {
        for y in 1..n as usize {
            dp[x][y] = dp[x-1][y] + dp[x][y-1];
        }
    }
    dp[(m - 1) as usize][(n -1) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(3, 7), 28);
        assert_eq!(unique_paths(2, 3), 3)
    }
}