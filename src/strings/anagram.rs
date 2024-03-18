use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut s = s;
    let mut s_map = HashMap::new();
    while let Some(c) = s.pop() {
        if let Some(v) = s_map.insert(c, 1) {
            s_map.insert(c, v + 1); 
        }
    }
    for c in t.chars() {
        let Some(v) = s_map.get(&c) else {
            return false;
        };
        if *v == 0 { return false };
        s_map.insert(c, v - 1);
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(is_anagram("anagram".to_string(), "nagaram".to_string()), true);
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
        assert_eq!(is_anagram("a".to_string(), "ab".to_string()), false);
    }
}