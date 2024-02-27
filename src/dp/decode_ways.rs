use std::collections::HashMap;

pub fn num_decodings(s: String) -> i32 {
    let mut dp: HashMap<usize, i32> = HashMap::new();
    let mut prev_char = '9';

    dp.insert(s.len() as usize, 1);
    
    for (i, c) in s.char_indices().rev() {
        if c == '0' {
            dp.insert(i, 0);
        } else {
            dp.insert(i, *dp.get(&(i + 1)).expect("Should already have been set"));
        }
        if i + 1 < s.len() && (c == '1' || (c == '2' && prev_char <= '6')) {
            let prev = dp.get(&(i + 2)).expect("Should already have been set");
            let cur = dp.get(&i).expect("Should already have been set");
            dp.insert(i, *prev + *cur);
        }
        prev_char = c
    }

    match dp.get(&0) {
        Some(r) => *r,
        None => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_decodings() {
        assert_eq!(num_decodings("12".to_string()), 2);
        assert_eq!(num_decodings("226".to_string()), 3);
        assert_eq!(num_decodings("0".to_string()), 0);
        assert_eq!(num_decodings("06".to_string()), 0);
        assert_eq!(num_decodings("10".to_string()), 1);
        assert_eq!(num_decodings("2101".to_string()), 1);
        assert_eq!(num_decodings("210".to_string()), 1);
        assert_eq!(num_decodings("2103".to_string()), 1);
        assert_eq!(num_decodings("2104".to_string()), 1);
        assert_eq!(num_decodings("2105".to_string()), 1);
        assert_eq!(num_decodings("2106".to_string()), 1);
        assert_eq!(num_decodings("2107".to_string()), 1);
        assert_eq!(num_decodings("2108".to_string()), 1);
        assert_eq!(num_decodings("2109".to_string()), 1);
        assert_eq!(num_decodings("2110".to_string()), 2);
        assert_eq!(num_decodings("2112".to_string()), 5);
        assert_eq!(num_decodings("2126".to_string()), 5);

    }
}