fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![vec![i32::MIN; k + 1]; cols]; rows];
    dp[0][0][0] = 0;

    for i in 0..rows {
        for j in 0..cols {
            for cost in 0..=k {
                if dp[i][j][cost] == i32::MIN {
                    continue;
                }

                if i + 1 < rows {
                    let new_cost = if grid[i + 1][j] == 0 { cost } else { cost + 1 };
                    if new_cost <= k {
                        dp[i + 1][j][new_cost] =
                            std::cmp::max(dp[i + 1][j][new_cost], dp[i][j][cost] + grid[i + 1][j]);
                    }
                }
                if j + 1 < cols {
                    let new_cost = if grid[i][j + 1] == 0 { cost } else { cost + 1 };
                    if new_cost <= k {
                        dp[i][j + 1][new_cost] =
                            std::cmp::max(dp[i][j + 1][new_cost], dp[i][j][cost] + grid[i][j + 1]);
                    }
                }
            }
        }
    }

    let mut ret = i32::MIN;
    for i in 0..=k {
        ret = std::cmp::max(ret, dp[rows - 1][cols - 1][i]);
    }

    if ret == i32::MIN { -1 } else { ret }
}

fn main() {
    let grid = vec![vec![0, 1], vec![2, 0]];
    let ret = max_path_score(grid, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![0, 1], vec![1, 2]];
        let ret = max_path_score(grid, 1);
        assert_eq!(ret, -1);
    }
    {
        let grid = vec![vec![0, 1], vec![2, 0]];
        let ret = max_path_score(grid, 1);
        assert_eq!(ret, 2);
    }
}
