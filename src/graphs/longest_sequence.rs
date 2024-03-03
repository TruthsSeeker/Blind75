use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut max = 0;

    let mut set = HashSet::new();
    let mut visited = HashSet::new();
    set.extend(nums.into_iter());
    for num in &set {
        if let Some(_) = set.get(&(num - 1)) {
            continue;
        }
        if visited.contains(num) {
            continue;
        }
        let mut sequence = 1;
        let mut n = *num;
        while let Some(i) = set.get(&(n + 1)) {
            visited.insert(i);
            sequence += 1;
            n += 1;
        }
        max = max.max(sequence);
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let expected = 4;
        assert_eq!(longest_consecutive(nums), expected);

        let nums = vec![0,3,7,2,5,8,4,6,0,1];
        let expected = 9;
        assert_eq!(longest_consecutive(nums), expected);
    }
}