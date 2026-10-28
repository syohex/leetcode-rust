fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![vec![false; 1024]; cols]; rows];
    dp[0][0][grid[0][0] as usize] = true;

    for i in 0..rows {
        for j in 0..cols {
            if i == 0 && j == 0 {
                continue;
            }

            for k in 0..1024 {
                let v = k ^ grid[i][j] as usize;
                if (i > 0 && dp[i - 1][j][k]) || (j > 0 && dp[i][j - 1][k]) {
                    dp[i][j][v] = true;
                }
            }
        }
    }

    for i in 0..1024 {
        if dp[rows - 1][cols - 1][i] {
            return i as i32;
        }
    }

    // never reach here
    -1
}

fn main() {
    let grid = vec![vec![1, 2], vec![3, 4]];
    let ret = min_cost(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let ret = min_cost(grid);
        assert_eq!(ret, 6);
    }
    {
        let grid = vec![vec![6, 7], vec![5, 8]];
        let ret = min_cost(grid);
        assert_eq!(ret, 9);
    }
}
