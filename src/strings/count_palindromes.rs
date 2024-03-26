pub fn count_substrings(s: String) -> i32 {
    let mut count = 0;
    let chars: Vec<_> = s.chars().collect();
    for i in 0..chars.len() {
        let (mut l, mut r) = (i, i);
        while r < s.len() && chars[l] == chars[r] {
            count += 1;
            if l == 0 { break; }
            l -= 1;
            r += 1;
        }

        (l, r) = (i, i+1);
        while r < s.len() && chars[l] == chars[r] {
            count += 1;
            if l == 0 { break; }
            l -= 1;
            r += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_substrings() {
        // Test case 1: non-palindromic string
        let s1 = String::from("abc");
        assert_eq!(count_substrings(s1), 3);

        // Test case 2: multi palindromic string
        let s2 = String::from("aaa");
        assert_eq!(count_substrings(s2), 6);
    }

}