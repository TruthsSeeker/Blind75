pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut sorted = intervals;
    sorted.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut result = vec![];

    let mut merged = vec![];

    for interval in &sorted {
        if merged.len() == 0 {
            merged = interval.clone();
            continue;
        }
        
        if merged[1] < interval[0] || merged[0] > interval[1] {
            result.push(merged);
            merged = interval.clone();
            continue;
        }
        merged[0] = merged[0].min(interval[0]);
        merged[1] = merged[1].max(interval[1]);
    }

    result.push(merged);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let intervals = vec![
            vec![1,3],
            vec![2,6],
            vec![8,10],
            vec![15,18]
        ];
        let expected = vec![
            vec![1,6],
            vec![8,10],
            vec![15,18]
        ];
        let result = merge(intervals);
        for interval in &result {
            assert!(expected.contains(&interval), "expected: {:?}, got: {:?}", expected, result);
        }

        let intervals = vec![
            vec![1,4],
            vec![4,5]
        ];
        let expected = vec![
            vec![1,5]
        ];
        let result = merge(intervals);
        for interval in &result {
            assert!(expected.contains(&interval), "expected: {:?}, got: {:?}", expected, result);
        }

        let intervals = vec![
            vec![1,4],
            vec![0,0]
        ];
        let expected = vec![
            vec![0,0],
            vec![1,4]
        ];
        let result = merge(intervals);
        for interval in &result {
            assert!(expected.contains(&interval), "expected: {:?}, got: {:?}", expected, result);
        }

        let intervals = vec![
            vec![2,3],
            vec![4,5],
            vec![6,7],
            vec![8,9],
            vec![1,10]
        ];
        let expected = vec![
            vec![1,10]
        ];
        let result = merge(intervals);
        for interval in &result {
            assert!(expected.contains(&interval), "expected: {:?}, got: {:?}", expected, result);
        }
    }
}
