use std::collections::HashMap;

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut memo = HashMap::new();
        find_subsequence_memo(text1.as_str(), text2.as_str(), &mut memo);
        find_subsequence_tab(text1, text2)
}

fn find_subsequence_memo(text1: &str, text2: &str, memo: &mut HashMap<(String, String), i32>) -> i32 {
    if text1.len() == 0 || text2.len() == 0 {
        return 0;
    }
    if let Some(m) = memo.get(&(text1.to_string(), text2.to_string())) {
        return *m
    }
    let mut max = 0;
    for (i, c) in text1.char_indices() {
        if let Some(j) = text2.find(c) {
            max = max.max(1 + find_subsequence_memo(&text1[i+1..], &text2[j+1..], memo));
            memo.insert((text1[..].to_string(), text2[..].to_string()), max);
        }
    }
    
    *memo.get(&(text1.to_string(), text2.to_string())).unwrap_or(&0)
}

fn find_subsequence_tab(text1: String, text2: String) -> i32 {
    let mut table = vec![vec![0; text2.chars().count() + 1]; text1.chars().count() + 1];

    for (i, c1) in text1.char_indices() {
        for (j, c2) in text2.char_indices() {
            if c1 == c2 {
                // Look to upper left to look at both previous indices
                table[i+1][j+1] = table[i][j] + 1;
            } else {
                table[i+1][j+1] = table[i+1][j].max(table[i][j+1]);
            }
        }
    }
    table[text1.chars().count()][text2.chars().count()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
        assert_eq!(longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
        assert_eq!(longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
        assert_eq!(longest_common_subsequence("".to_string(), "def".to_string()), 0);
        assert_eq!(longest_common_subsequence("".to_string(), "".to_string()), 0);
        assert_eq!(longest_common_subsequence("oxcpqrsvwf".to_string(), "shmtulqrypy".to_string()), 2);
        assert_eq!(longest_common_subsequence("jgtargjctqvijshexyjcjcre".to_string(), "pyzazexujqtsjebcnadahobwf".to_string()), 6);

    }
}