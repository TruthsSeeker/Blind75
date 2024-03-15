use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    let s: Vec<char> = s.chars().collect();
    let mut t_map = HashMap::new();
    for c in t.chars() {
        if let Some(v) = t_map.insert(c, 1) {
            t_map.insert(c, v + 1);
        } 
    }
    let test = |t_map: &HashMap<char, i32>, window_map: &HashMap<char, i32>| -> bool {
        for (k, v) in t_map {
            let Some(w) = window_map.get(&k) else {
                return false;
            };
            if v > w {
                return false;
            }
        }
        true
    };
    
    let mut window_map = HashMap::new();
    let mut min_window = (0 as usize, usize::MAX);
    let mut left = 0;
    
    for (right, s_c) in s.iter().enumerate() {
        if let Some(v) = window_map.insert(*s_c, 1) {
            window_map.insert(*s_c, v + 1);
        }
        while test(&t_map, &window_map) {
            if right - left < min_window.1 - min_window.0 {
                min_window = (left, right);
            }
            if let Some(v) = window_map.get(&s[left]) {
                window_map.insert(s[left], v - 1);
            }
            left += 1;
        }
    }

    if min_window.1 - min_window.0 < usize::MAX {
        let s: String = s[min_window.0..=min_window.1].to_vec().iter().collect();
        return s;
    }

    "".to_string()       
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a".to_string());
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "".to_string());
    }
}