pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut prev_end = intervals[0][1];
    for i in 1..intervals.len() {
        if intervals[i][0] >= prev_end {
            prev_end = intervals[i][1];
        } else {
            count += 1;
            prev_end = prev_end.min(intervals[i][1])
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_erase_overlap_intervals() {
        let intervals = vec![
            vec![1,2],
            vec![2,3],
            vec![3,4],
            vec![1,3]
        ];
        assert_eq!(erase_overlap_intervals(intervals), 1);

        let intervals = vec![
            vec![1,2],
            vec![1,2],
            vec![1,2]
        ];
        assert_eq!(erase_overlap_intervals(intervals), 2);

        let intervals = vec![
            vec![1,2],
            vec![2,3]
        ];
        assert_eq!(erase_overlap_intervals(intervals), 0);

        let intervals = vec![
            vec![0,2],
            vec![1,3],
            vec![2,4],
            vec![3,5],
            vec![4,6]
        ];
        assert_eq!(erase_overlap_intervals(intervals), 2);
    }
}
