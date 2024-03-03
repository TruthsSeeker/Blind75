use std::collections::HashSet;

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    let mut visited = HashSet::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {            
            if dfs(row as i32, col as i32, &mut visited, &grid) {
                count += 1;
            }
        }
    }
    
    count
}

fn dfs(row: i32, col: i32, visited: &mut HashSet<(i32, i32)>, grid: &Vec<Vec<char>>) -> bool {
    if visited.contains(&(row, col)) 
    || !(0 <= row && row < grid.len() as i32) 
    || !(0 <= col && col < grid[0].len() as i32) 
    || grid[row as usize][col as usize] == '0' {
        return false;
    }
    visited.insert((row, col));

    dfs(row - 1, col, visited, grid);
    dfs(row + 1, col, visited, grid);
    dfs(row, col - 1, visited, grid);
    dfs(row, col + 1, visited, grid);

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands() {
        let grid = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0']
        ];
        let expected = 1;
        assert_eq!(num_islands(grid), expected);
    }
}
