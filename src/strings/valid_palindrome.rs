pub fn is_palindrome(s: String) -> bool {
    let mut back_half = vec![];

    // Sanitize s
    let mut s = s;
    s = s.chars()
        .filter(|c| {
            c.is_alphanumeric()
        }).map(|c| {
            c.to_ascii_lowercase()
    }).collect();
    
    let middle = s.len() / 2;
    let is_even = s.len() % 2 == 0;
    for (i, c) in s.char_indices() {
        if i < middle {
            back_half.push(c);
        } else {
            if i == middle && !is_even {
                continue;
            }
            match back_half.pop() {
                Some(b) => if c != b {
                    return false
                },
                None => return false,
            }
        }
    }
    
    back_half.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        // Test cases for valid palindromes
        assert_eq!(is_palindrome(String::from("A man, a plan, a canal, Panama")), true);
        assert_eq!(is_palindrome(String::from("racecar")), true);
        assert_eq!(is_palindrome(String::from("level")), true);
        assert_eq!(is_palindrome(String::from("12321")), true);
        assert_eq!(is_palindrome(String::from("    ")), true);

        // Test cases for invalid palindromes
        assert_eq!(is_palindrome(String::from("hello")), false);
        assert_eq!(is_palindrome(String::from("world")), false);
        
    }
}