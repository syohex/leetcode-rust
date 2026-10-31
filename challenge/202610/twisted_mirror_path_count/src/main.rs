fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
    let modulo = 1_000_000_007i64;
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![[0i64; 2]; cols]; rows];
    dp[0][0][0] = 1;

    for i in 0..rows {
        for j in 0..cols {
            if i > 0 {
                if grid[i - 1][j] == 1 {
                    dp[i][j][0] = dp[i - 1][j][1];
                } else {
                    dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1]) % modulo;
                }
            }
            if j > 0 {
                if grid[i][j - 1] == 1 {
                    dp[i][j][1] = dp[i][j - 1][0];
                } else {
                    dp[i][j][1] = (dp[i][j - 1][0] + dp[i][j - 1][1]) % modulo;
                }
            }
        }
    }

    ((dp[rows - 1][cols - 1][0] + dp[rows - 1][cols - 1][1]) % modulo) as i32
}

fn main() {
    let grid = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]];
    let ret = unique_paths(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]];
        let ret = unique_paths(grid);
        assert_eq!(ret, 5);
    }
    {
        let grid = vec![vec![0, 0], vec![0, 0]];
        let ret = unique_paths(grid);
        assert_eq!(ret, 2);
    }
    {
        let grid = vec![vec![0, 1, 1], vec![1, 1, 0]];
        let ret = unique_paths(grid);
        assert_eq!(ret, 1);
    }
}
