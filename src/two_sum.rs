use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result: Vec<i32> = vec![];
    for i in 0..nums.len() { 
        if let Some(j) = map.get(&nums[i]) {
            result.push(j.clone());
            result.push(i as i32);
            break;
        }
        map.insert(target - nums[i], i as i32);
    }
    result
}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}

