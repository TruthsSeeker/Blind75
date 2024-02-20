pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len().saturating_sub(1);
    let mut sorted: Vec<i32> = vec![];
    let mut pivot = 0;
    // Start by re-sorting the array by finding the pivot
    if nums[l] <= nums[r] {
        sorted = nums.clone();
    } else {
        while l <= r {
            let m = (l + r) / 2;

            if nums[m] > nums[r] {
                l = m;
            } else if nums[l] > nums[m] {
                r = m;
            }

            if nums[m] > nums[m + 1] {
                sorted = [nums[m + 1..nums.len()].to_vec(), nums[0..=m].to_vec()].concat();
                pivot = m as i32 + 1;
                break;
            }
        }
        l = 0;
        r = sorted.len().saturating_sub(1);
    }

    // Binary search the sorted array to find the sorted index
    let mut sorted_index = -1;
    while l <= r {
        let m = (l + r) / 2;
        if sorted[m] < target {
            l = m + 1
        } else if sorted[m] > target {
            r = m.saturating_sub(1);
        } else {
            sorted_index = m as i32;
            break;
        }
        if l == r {
            if sorted[l] == target {
                sorted_index = l as i32;
            } else {
                return -1
            };
            break;
        }
    }
    let result = (sorted_index + pivot) % nums.len() as i32;
    if nums[result as usize] == target {
        result
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(search(vec![3, 5, 1], 3), 0);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 0), 4);
        assert_eq!(search(vec![4,5,6,7,0,1,2], 3), -1);
        assert_eq!(search(vec![1], 0), -1);
        assert_eq!(search(vec![1], 1), 0);
        assert_eq!(search(vec![1, 3], 3), 1);
        assert_eq!(search(vec![1, 3], 1), 0);
        assert_eq!(search(vec![1, 3], 0), -1);
        assert_eq!(search(vec![3, 1], 1), 1);
        assert_eq!(search(vec![3, 1], 3), 0);
        assert_eq!(search(vec![3, 4, 5, 1, 2], 1), 3);
        assert_eq!(search(vec![3, 4, 5, 1, 2], 2), 4);
        assert_eq!(search(vec![5,1,2,3,4], 1), 1);
    }
}