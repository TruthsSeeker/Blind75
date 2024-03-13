use std::collections::HashSet;

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let word: Vec<char> = word.chars().collect();
    
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let mut set = HashSet::new();
            if dfs(&board, &word, i as i32, j as i32, 0, &mut set) {
                return true;
            }
        }
    }
    false
}

fn dfs(board: &Vec<Vec<char>>, word: &Vec<char>, r: i32, c:i32, p: usize, visited: &mut HashSet<(i32, i32)>) -> bool {
    if r < 0 || c < 0
        || r > board.len() as i32 - 1 || c > board[0].len() as i32 - 1
        || visited.contains(&(r,c))  {
            return false;
    }
    if board[r as usize][c as usize] != word[p] {
        return false;
    }
    visited.insert((r,c));
    if p == word.len() - 1 {
        return true;
    }
    let res = dfs(board, word, r, c - 1, p + 1, visited) ||
    dfs(board, word, r - 1, c, p + 1, visited) ||
    dfs(board, word, r + 1, c, p + 1, visited) ||
    dfs(board, word, r, c + 1, p + 1, visited);
    visited.remove(&(r,c));
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exist() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        assert_eq!(exist(board.clone(), "ABCCED".to_string()), true);
        assert_eq!(exist(board.clone(), "SEE".to_string()), true);
        assert_eq!(exist(board.clone(), "ABCB".to_string()), false);

        let board = vec![
            vec!['C','A','A'],
            vec!['A','A','A'],
            vec!['B','C','D']
        ];
        assert_eq!(exist(board.clone(), "AAB".to_string()), true);

        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','E','S'],
            vec!['A','D','E','E']
        ];
        assert_eq!(exist(board.clone(), "ABCESEEEFS".to_string()), true);
    }
}
