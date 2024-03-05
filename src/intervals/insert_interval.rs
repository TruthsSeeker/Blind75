pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    if intervals.len() == 0 {
        return vec![new_interval];
    }

    let mut result = vec![];
    let mut new_interval = new_interval;

    for i in 0..intervals.len() {
        if intervals[i][0] > new_interval[1] {
            result.push(new_interval.clone());
            result.append(&mut intervals[i..].to_vec());
            return result;
        } else if intervals[i][1] < new_interval[0] {
            result.push(intervals[i].clone());
            continue;
        } else {
            new_interval[0] = new_interval[0].min(intervals[i][0]);
            new_interval[1] = new_interval[1].max(intervals[i][1])
        }
    }
    result.push(new_interval);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let intervals = vec![
            vec![1,3],
            vec![6,9]
        ];
        let new_interval = vec![2,5];
        let expected = vec![
            vec![1,5],
            vec![6,9]
        ];
        assert_eq!(insert(intervals, new_interval), expected);

        let intervals = vec![
            vec![1,2],
            vec![3,5],
            vec![6,7],
            vec![8,10],
            vec![12,16]
        ];
        let new_interval = vec![4,8];
        let expected = vec![
            vec![1,2],
            vec![3,10],
            vec![12,16]
        ];
        assert_eq!(insert(intervals, new_interval), expected);

        let intervals = vec![];
        let new_interval = vec![5,7];
        let expected = vec![
            vec![5,7]
        ];
        assert_eq!(insert(intervals, new_interval), expected);

        let intervals = vec![
            vec![1,5]
        ];
        let new_interval = vec![2,3];
        let expected = vec![
            vec![1,5]
        ];
        assert_eq!(insert(intervals, new_interval), expected);

        let intervals = vec![
            vec![1,5]
        ];
        let new_interval = vec![2,7];
        let expected = vec![
            vec![1,7]
        ];
        assert_eq!(insert(intervals, new_interval), expected);
    }
}