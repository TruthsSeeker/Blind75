use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut strs_map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
    for str in strs {
        
        let mut str_count = vec![0; 26];
        for c in str.chars() {
            let index = (c as u8 - 'a' as u8) as usize;
            str_count[index] += 1;
        }
        if let Some(v) = strs_map.get_mut(&str_count) {
            v.push(str);
        } else {
            strs_map.insert(str_count, vec![str]);
        }
    }

    strs_map.into_iter().map(|(_, v)| v).collect() 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let input = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
        let expected = vec![
            vec!["bat".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()], 
        ];
        assert_eq!(group_anagrams(input), expected);
    }
}