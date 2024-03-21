pub fn is_valid(s: String) -> bool {
    let mut closing_stack = vec![];
    for c in s.chars() {
        match c {
            '(' => closing_stack.push(')'),
            '[' => closing_stack.push(']'),
            '{' => closing_stack.push('}'),
            closing @ ')' | closing @ ']' | closing @ '}' => match closing_stack.pop() {
                Some(top) => if top != closing {
                    return false
                },
                None => return false,
            }
           _ => return false 
        }
    }
    
    closing_stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        // Test cases for valid parentheses
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("{[]}".to_string()), true);
        assert_eq!(is_valid("((()))".to_string()), true);
        assert_eq!(is_valid("[[[]]]".to_string()), true);
        assert_eq!(is_valid("{{}}".to_string()), true);

        // Test cases for invalid parentheses
        assert_eq!(is_valid("(".to_string()), false);
        assert_eq!(is_valid(")".to_string()), false);
        assert_eq!(is_valid("([)]".to_string()), false);
        assert_eq!(is_valid("{[}]".to_string()), false);
        assert_eq!(is_valid("((())".to_string()), false);
        assert_eq!(is_valid("[[[]]".to_string()), false);
        assert_eq!(is_valid("{{}".to_string()), false);
    }
}