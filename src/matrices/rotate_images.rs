
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let mut start = 0;
    let mut end = matrix.len() - 1;
    let middle =  matrix.len()/2;

    for _ in 0..middle {
        for i in 0..(end - start) {
            (
                matrix[start + i][end], 
                matrix[end][end - i], 
                matrix[end - i][start], 
                matrix[start][start + i]
            ) = (
                matrix[start][start + i], 
                matrix[start + i][end], 
                matrix[end][end - i], 
                matrix[end - i][start]
            );
        }
        start += 1;
        end -= 1;
    }


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        rotate(&mut matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3]
        ]);

        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16]
        ];
        rotate(&mut matrix);
        assert_eq!(matrix, vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11]
        ]);
    }
}