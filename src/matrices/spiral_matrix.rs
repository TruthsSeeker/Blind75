pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut left = 0;
    let mut right = matrix[0].len() - 1;
    let mut top = 0;
    let mut bottom = matrix.len() - 1;
    let mut current_row = 0;
    let mut current_col = 0;
    
    while result.len() != matrix.len() * matrix[0].len() {
        if current_col == left {
            if current_row == top {
                for i in left..=right {
                    result.push(matrix[current_row][i]);
                    current_col = i;
                }
                current_row += 1;
                top += 1;
            } else {
                for j in (top..=bottom).rev() {
                    result.push(matrix[j][current_col]);
                    current_row = j;
                }
                current_col += 1;
                left += 1;
            }
        } else {
            if current_row == top {
                for j in top..=bottom {
                    result.push(matrix[j][current_col]);
                    current_row = j;
                }
                current_col -= 1;
                right -= 1;
            } else {
                for i in (left..=right).rev() {
                    result.push(matrix[current_row][i]);
                    current_col = i;
                }
                current_row -= 1;
                bottom -= 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spiral_order() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        assert_eq!(spiral_order(matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);

        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ];
        assert_eq!(spiral_order(matrix), vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }
}