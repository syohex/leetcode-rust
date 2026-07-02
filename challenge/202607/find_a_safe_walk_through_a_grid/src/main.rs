fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
    use std::collections::BinaryHeap;

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![-1; cols]; rows];

    let mut q = BinaryHeap::new();
    q.push((health - grid[0][0], 0, 0));

    let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while !q.is_empty() {
        let (h, row, col) = q.pop().unwrap();
        if h <= 0 || dp[row as usize][col as usize] != -1 {
            continue;
        }

        dp[row as usize][col as usize] = h;

        for &s in &steps {
            let (r, c) = (row + s.0, col + s.1);
            if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
                q.push((h - grid[r as usize][c as usize], r, c));
            }
        }
    }

    dp[rows - 1][cols - 1] >= 1
}

fn main() {
    let grid = vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 0, 0, 1, 0],
    ];
    let ret = find_safe_walk(grid, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1, 1, 1, 1]];
        let ret = find_safe_walk(grid, 4);
        assert!(!ret);
    }
    {
        let grid = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0],
        ];
        let ret = find_safe_walk(grid, 1);
        assert!(ret);
    }
    {
        let grid = vec![
            vec![0, 1, 1, 0, 0, 0],
            vec![1, 0, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 0, 1],
            vec![0, 0, 1, 0, 1, 0],
        ];
        let ret = find_safe_walk(grid, 3);
        assert!(!ret);
    }
    {
        let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let ret = find_safe_walk(grid, 5);
        assert!(ret);
    }
}
