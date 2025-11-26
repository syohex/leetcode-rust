fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let modulo = 1_000_000_007;
    let mut dp = vec![vec![vec![0; k as usize]; cols + 1]; rows + 1];

    for i in 0..rows {
        for j in 0..cols {
            if i == 0 && j == 0 {
                let m = grid[0][0] % k;
                dp[1][1][m as usize] = 1;
                continue;
            }

            for n in 0..k {
                let m = grid[i][j] % k;
                let prev = (n + k - m) % k;
                dp[i + 1][j + 1][n as usize] =
                    (dp[i][j + 1][prev as usize] + dp[i + 1][j][prev as usize]) % modulo;
            }
        }
    }

    dp[rows][cols][0]
}

fn main() {
    let grid = vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]];
    let ret = number_of_paths(grid, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]];
        let ret = number_of_paths(grid, 3);
        assert_eq!(ret, 2);
    }
    {
        let grid = vec![vec![0, 0]];
        let ret = number_of_paths(grid, 5);
        assert_eq!(ret, 1);
    }
    {
        let grid = vec![vec![7, 3, 4, 9], vec![2, 3, 6, 2], vec![2, 3, 7, 0]];
        let ret = number_of_paths(grid, 1);
        assert_eq!(ret, 10);
    }
}
