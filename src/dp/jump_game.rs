use std::collections::HashSet;

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut set: HashSet<usize> = HashSet::new();
    set.insert(0);

    for i in 0..nums.len() {
        if let Some(b) = set.get(&i).copied() {
            
            for j in 1..=nums[i] {
                if i + j as usize == nums.len() - 1 {
                    return true;
                }
                set.insert(b + j as usize);
            }

        }
    }

    if let Some(_) = set.get(&(nums.len() - 1)) { true } else { false }
}

pub fn can_jump_dp(nums: Vec<i32>) -> bool {
    let mut reach = 0;
    for n in nums {
        if reach < 0 {
            return false;
        }
        if n > reach {
            reach = n;
        }
        reach -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump() {
        assert!(can_jump(vec![2,3,1,1,4]));
        assert!(!can_jump(vec![3,2,1,0,4]));
    }

    #[test]
    fn test_can_jump_dp() {
        assert!(can_jump_dp(vec![2,3,1,1,4]));
        assert!(!can_jump_dp(vec![3,2,1,0,4]));
    }
}