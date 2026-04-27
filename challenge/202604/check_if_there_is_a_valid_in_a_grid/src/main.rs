fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
    use std::collections::VecDeque;
    let dirs = [
        [(0, -1), (0, 1)],
        [(-1, 0), (1, 0)],
        [(0, -1), (1, 0)],
        [(1, 0), (0, 1)],
        [(0, -1), (-1, 0)],
        [(-1, 0), (0, 1)],
    ];

    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
    let mut visited = vec![vec![false; cols as usize]; rows as usize];

    let mut q = VecDeque::new();
    q.push_back((0i32, 0i32));
    visited[0][0] = true;

    while !q.is_empty() {
        let (row, col) = q.pop_front().unwrap();
        let idx = (grid[row as usize][col as usize] - 1) as usize;
        for (x, y) in &dirs[idx] {
            let (r, c) = (row + x, col + y);
            if r >= 0 && r < rows && c >= 0 && c < cols && !visited[r as usize][c as usize] {
                let next_idx = (grid[r as usize][c as usize] - 1) as usize;
                for next in &dirs[next_idx] {
                    if r + next.0 == row && c + next.1 == col {
                        visited[r as usize][c as usize] = true;
                        q.push_back((r, c));
                    }
                }
            }
        }
    }

    visited[rows as usize - 1][cols as usize - 1]
}

fn main() {
    let grid = vec![vec![2, 4, 3], vec![6, 5, 2]];
    let ret = has_valid_path(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1]];
        let ret = has_valid_path(grid);
        assert!(ret);
    }
    {
        let grid = vec![vec![2, 4, 3], vec![6, 5, 2]];
        let ret = has_valid_path(grid);
        assert!(ret);
    }
    {
        let grid = vec![vec![1, 2, 1], vec![1, 2, 1]];
        let ret = has_valid_path(grid);
        assert!(!ret);
    }
    {
        let grid = vec![vec![1, 1, 2]];
        let ret = has_valid_path(grid);
        assert!(!ret);
    }
}
