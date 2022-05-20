fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let rows = obstacle_grid.len();
    let cols = obstacle_grid[0].len();

    let mut dp = vec![vec![0; cols]; rows];
    if obstacle_grid[0][0] == 0 {
        dp[0][0] = 1;
    }

    for i in 0..rows {
        for j in 0..cols {
            if obstacle_grid[i][j] == 1 {
                continue;
            }

            if i >= 1 {
                dp[i][j] += dp[i - 1][j];
            }
            if j >= 1 {
                dp[i][j] += dp[i][j - 1];
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
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 2);
    }
    {
        let obstacle_grid = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 1);
    }
    {
        let obstacle_grid = vec![vec![1]];
        assert_eq!(unique_paths_with_obstacles(obstacle_grid), 0);
    }
}
