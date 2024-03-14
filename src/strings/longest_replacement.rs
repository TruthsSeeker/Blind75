use std::collections::HashMap;

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut res = 0;
    let s: Vec<char> = s.chars().collect();
    let mut count = HashMap::new();

    let mut left = 0;
    let mut max_frequency = 0;

    for i in 0..s.len() {
        // increment the count of the character at i
        count.insert(s[i], 1 + count.get(&s[i]).unwrap_or(&0));
        // update the max frequency
        max_frequency = max_frequency.max(*count.get(&s[i]).unwrap_or(&0));

        // move the left pointer until the window is valid
        while (i - left + 1) as i32 - max_frequency > k {
            // update the count ouf the character at left
            count.insert(s[left], count.get(&s[left]).unwrap_or(&0) - 1);
            left += 1;
        }
        // update result
        res = res.max((i - left + 1) as i32);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_character_replacement() {
        assert_eq!(character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(character_replacement("AABABBA".to_string(), 1), 4);
        assert_eq!(character_replacement("AABABBA".to_string(), 0), 2);
        assert_eq!(character_replacement("AABABBA".to_string(), 3), 7);
        assert_eq!(character_replacement("ABBB".to_string(), 2), 4);
        assert_eq!(character_replacement("AABBB".to_string(), 2), 5);
    }
}