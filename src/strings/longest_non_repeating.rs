use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let s: Vec<char> = s.chars().collect();
    let mut max = 0;
    let mut set = HashSet::new();

    for i in 0..s.len() {
        while set.len() + i < s.len() {
            if set.contains(&s[i+set.len()]) {
                break;
            }
            set.insert(s[i+set.len()]);
            max = max.max(set.len());
        }
        if set.len() + i == s.len() {
            return max as i32;
        }
        if s[i] != s[i+set.len()] {
            set.remove(&s[i]);
        }
        max = max.max(set.len());
    }
    
    max as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
        assert_eq!(length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
        assert_eq!(length_of_longest_substring("cdd".to_string()), 2);
    }
}