use std::collections::{HashMap, HashSet};

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
    let mut squares: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] != '.' {
                match rows.get(&i) {
                    Some(s) => {
                        if s.get(&board[i][j]).is_some() {
                            return false;
                        }
                    }
                    None => {}
                }

                match cols.get(&j) {
                    Some(s) => {
                        if s.get(&board[i][j]).is_some() {
                            return false;
                        }
                    }
                    None => {}
                }

                match squares.get(&(i / 3, j / 3)) {
                    Some(s) => {
                        if s.get(&board[i][j]).is_some() {
                            return false;
                        }
                    }
                    None => {}
                }

                rows.entry(i)
                    .and_modify(|s| {
                        s.insert(board[i][j]);
                    })
                    .or_insert(HashSet::from([board[i][j]]));

                cols.entry(j)
                    .and_modify(|s| {
                        s.insert(board[i][j]);
                    })
                    .or_insert(HashSet::from([board[i][j]]));

                squares
                    .entry((i / 3, j / 3))
                    .and_modify(|s| {
                        s.insert(board[i][j]);
                    })
                    .or_insert(HashSet::from([board[i][j]]));
            }
        }
    }

    return true;
}
