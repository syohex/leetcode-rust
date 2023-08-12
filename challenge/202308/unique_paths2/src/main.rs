fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    let (rows, cols) = (obstacle_grid.len(), obstacle_grid[0].len());
    let mut dp = vec![vec![0; cols]; rows];
    let mut checked = vec![vec![false; cols]; rows];

    let steps = vec![(1, 0), (0, 1)];
    let mut q = VecDeque::new();

    if obstacle_grid[0][0] == 0 {
        dp[0][0] = 1;
        checked[0][0] = true;
        q.push_back((0, 0));
    }

    while !q.is_empty() {
        let len = q.len();
        for _ in 0..len {
            let (r, c) = q.pop_front().unwrap();
            for (x, y) in &steps {
                let row = r + *x;
                let col = c + *y;
                if row < rows && col < cols && obstacle_grid[row][col] == 0 {
                    dp[row][col] += dp[r][c];
                    if !checked[row][col] {
                        checked[row][col] = true;
                        q.push_back((row, col));
                    }
                }
            }
        }
    }

    dp[rows - 1][cols - 1]
}

fn main() {
    let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    let ret = unique_paths_with_obstacles(obstacle_grid);
    println!("ret={ret}");
}

#[test]
fn test_unique_paths_with_obstacles() {
    {
        let obstacle_grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let ret = unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(ret, 2);
    }
    {
        let obstacle_grid = vec![vec![0, 1], vec![0, 0]];
        let ret = unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(ret, 1);
    }
    {
        let obstacle_grid = vec![vec![1]];
        let ret = unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(ret, 0);
    }
    {
        let obstacle_grid = vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        let ret = unique_paths_with_obstacles(obstacle_grid);
        assert_eq!(ret, 7);
    }
}
