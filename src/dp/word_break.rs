use std::collections::HashSet;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut memo = HashSet::new();
    word_break_memo(&s, &word_dict, &mut memo)
}

fn word_break_memo(s: &String, word_dict: &Vec<String>, memo: &mut HashSet<String>) -> bool {
    if s.len() == 0 {
        return true;
    }
    if memo.contains(s) {
        return false;
    }

    for word in word_dict {
        if let Some(sub) = s.strip_prefix(word) {
            if word_break_memo(&sub.to_string(), word_dict, memo) {
                return true;
            } else {
                memo.insert(s.clone());
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert_eq!(word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]), true);
        assert_eq!(word_break("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]), true);
        assert_eq!(word_break("catsandog".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), false);
        assert_eq!(word_break("".to_string(), vec!["cats".to_string(), "dog".to_string(), "sand".to_string(), "and".to_string(), "cat".to_string()]), true);
    }
}