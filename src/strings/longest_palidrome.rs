pub fn longest_palindrome(s: String) -> String {
    let mut res = "";
    let mut res_len = 0;
    let mut chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len()  {
        
        let (mut l, mut r) = (i, i);
        while r < s.len() && chars[l] == chars[r] {
            if (r - l + 1) > res_len {
                res = &s[l..=r];
                res_len = r - l + 1;
            }
            if l == 0 { break; }
            l -= 1;
            r += 1;
        }
        
        (l, r) = (i, i+1);
        while l >= 0 && r < s.len() && chars[l] == chars[r] {
            if (r - l + 1) > res_len {
                res = &s[l..=r];
                res_len = r - l + 1;
            }
            if l == 0 { break; }
            l -= 1;
            r += 1;
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        // Test case 1: Palindrome with odd length
        let s1 = String::from("level");
        assert_eq!(longest_palindrome(s1), "level");

        // Test case 2: Palindrome with even length
        let s2 = String::from("noon");
        assert_eq!(longest_palindrome(s2), "noon");

        // Test case 3: Empty string
        let s3 = String::from("");
        assert_eq!(longest_palindrome(s3), "");

        // Test case 4: Single character
        let s4 = String::from("a");
        assert_eq!(longest_palindrome(s4), "a");

        // Test case 5: Multiple palindromes
        let s5 = String::from("racecarlevelnoon");
        assert_eq!(longest_palindrome(s5), "racecar");
    }
}