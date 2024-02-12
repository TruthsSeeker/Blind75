use std::collections::HashMap;

pub fn climb_stairs(n: i32) -> i32 {
    let mut memo: HashMap<i32, i32> = HashMap::new();
    climb(n, &mut memo)
}

pub fn climb(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        1
    } else if let Some(m) = memo.get(&n) {
        *m
    } else {
        let result = climb(n - 1, memo) + climb(n -2, memo);
        memo.insert(n, result);
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
    }
}