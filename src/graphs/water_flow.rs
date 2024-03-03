use std::collections::HashSet;

pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let rows = heights.len();
    let cols = heights[0].len();

    let mut pacific_shore = HashSet::new();
    let mut atlantic_shore = HashSet::new();

    for c in 0..cols {
        dfs(0, c as i32, heights[0][c], &mut pacific_shore, &heights);
        dfs(rows as i32 - 1, c as i32, heights[rows - 1][c], &mut atlantic_shore, &heights);
    }
    
    for r in 0..rows {
        dfs(r as i32, 0 as i32, heights[r][0], &mut pacific_shore, &heights);
        dfs(r as i32, cols as i32 - 1, heights[r][cols - 1], &mut atlantic_shore, &heights);
    }

    let both = atlantic_shore.intersection(&pacific_shore);

    both.cloned()
    .map(|(row, col)| vec![row as i32, col as i32])
    .collect()
}

fn dfs(row: i32, col: i32, prev_height: i32, visited: &mut HashSet<(i32, i32)>, heights: &Vec<Vec<i32>>) {
    if visited.contains(&(row, col)) 
    || row < 0 || col < 0
    || row >= heights.len() as i32 || col >= heights[0].len() as i32 
    || heights[row as usize][col as usize] < prev_height{
        return;
    }
    visited.insert((row, col));

    dfs(row + 1, col, heights[row as usize][col as usize], visited, heights);
    dfs(row - 1, col, heights[row as usize][col as usize], visited, heights);
    dfs(row, col + 1, heights[row as usize][col as usize], visited, heights);
    dfs(row, col - 1, heights[row as usize][col as usize], visited, heights);
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacific_atlantic() {
        let heights = vec![
            vec![1,2,2,3,5],
            vec![3,2,3,4,4],
            vec![2,4,5,3,1],
            vec![6,7,1,4,5],
            vec![5,1,1,2,4]
        ];
        let expected = vec![
            vec![0,4],
            vec![1,3],
            vec![1,4],
            vec![2,2],
            vec![3,0],
            vec![3,1],
            vec![4,0]
        ];
        let results = pacific_atlantic(heights);
        println!("{:?}", results);
        for expect in expected {
            assert!(results.contains(&expect))
        }
    }
}