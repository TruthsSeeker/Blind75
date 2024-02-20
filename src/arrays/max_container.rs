pub fn max_area(height: Vec<i32>) -> i32 {
    if height.len() < 2 { return 0 };
    let mut max = 0;
    let mut low = 0;
    let mut high = height.len() - 1;

    while low <= high {
        let width = (high - low) as i32;
        let area = height[low].min(height[high]) * width;
        if max < area { max = area }
        if height[low] > height[high] {
            high -= 1;
        } else if height[low] <= height[high] {
            low += 1;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(max_area(vec![1,1]), 1);
        assert_eq!(max_area(vec![4,3,2,1,4]), 16);
        assert_eq!(max_area(vec![1,2,1]), 2);
    }
}