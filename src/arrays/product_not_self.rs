
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    let mut acc = 1;
    let product_up_to: Vec<i32> = nums.iter().map(|n| {
        acc *= n;
        acc
    }).collect();
    acc = 1;

    let product_from: Vec<i32> = nums.iter().rev().map( |n| {
        acc *= n;
        acc
    }).collect();

    for i in 0..nums.len() {
        if i == 0 {
            result.push(product_from[nums.len() - 2])
        } else if i == nums.len() - 1 {
            result.push(product_up_to[nums.len() - 2])
        } else {
            result.push(product_up_to[i - 1] * product_from[nums.len() - 2 - i])
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
        assert_eq!(product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
    }
}
