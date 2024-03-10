pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut top_column = false;
    for row in 0..matrix.len() {
        for column in 0..matrix[row].len() {
            if matrix[row][column] == 0 {
                if column == 0 {
                    top_column = true;
                    matrix[row][0] = 0
                } else {
                    matrix[0][column] = 0;
                    matrix[row][0] = 0
                }
            }
        }
    }

    println!("{:?}", matrix);
    println!("{:?}", top_column);

    for i in 1..matrix[0].len() {
        if matrix[0][i] == 0 {
            for j in 0..matrix.len() {
                matrix[j][i] = 0;
            }
        }
    }
    for j in 0..matrix.len() {
        if matrix[j][0] == 0 {
            for i in 0..matrix[j].len() {
                matrix[j][i] = 0;
            }
        }
    }
    if top_column {
        for j in 0..matrix.len() {
            matrix[j][0] = 0;
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_zeroes() {
        let mut matrix = vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1]
        ];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![1, 0, 1]
        ]);

        let mut matrix = vec![
            vec![0, 1, 2, 0],
            vec![3, 4, 5, 2],
            vec![1, 3, 1, 5]
        ];
        set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![
            vec![0, 0, 0, 0],
            vec![0, 4, 5, 0],
            vec![0, 3, 1, 0]
        ]);
    }
}