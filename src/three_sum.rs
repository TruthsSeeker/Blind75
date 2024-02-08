use std::collections::{HashMap, HashSet};


pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut consumed = HashSet::new();

    for i in 0..nums.len() {
        let mut search_nums = nums.clone();
        search_nums.remove(i);
        for two_sum in all_two_sums(&search_nums, -nums[i]) {
            let mut triplet = vec![nums[i]];
            triplet.extend(two_sum);
            triplet.sort();
            if !consumed.contains(&triplet) {
                consumed.insert(triplet.clone());
                result.push(triplet);
            }

        }
    }

    println!("{:?}", result);
    result
}

fn all_two_sums(nums: &Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        if !map.contains_key(&(target - nums[i])) {
            map.insert(target - nums[i], i);
        }
    }
    let mut consumed = vec![];
    for k in 0..nums.len() {
        if let Some(j) = map.get(&nums[k]) {
            if !consumed.contains(&nums[k]) && j != &k {
                consumed.push(nums[*j]);
                consumed.push(nums[k]);

                result.push(vec![nums[k], nums[*j]])
            }
        }  
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let mut left:Vec<i32> = three_sum(vec![-1, 0, 1, 2, -1, -4]).into_iter().flatten().collect();
        left.sort();
        assert_eq!(left, vec![-1, -1, -1, 0, 1, 2]);

        left = three_sum(vec![0, 0, 0]).into_iter().flatten().collect();
        left.sort();
        assert_eq!(left, vec![0, 0, 0]);

        left = three_sum(vec![0, 1, 1]).into_iter().flatten().collect();
        left.sort();
        assert_eq!(left, vec![]);
    }

    #[test]
    fn test_all_two_sums() {
        let mut left:Vec<i32> = all_two_sums(&vec![-1, 0, 1, 2, -1, -4], 1).into_iter().flatten().collect();
        left.sort();
        assert_eq!(left, vec![-1, 0, 1, 2]);
        

        left = all_two_sums(&vec![0, 0], 0).into_iter().flatten().collect();
        left.sort();
        assert_eq!(left, vec![0, 0]);

        left = all_two_sums(&vec![1, 1], 1).into_iter().flatten().collect();
        left.sort();
        assert_eq!(left, vec![]);


    }
}