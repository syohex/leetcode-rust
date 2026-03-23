fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut maxs = vec![vec![0i64; cols]; rows];
    let mut mins = vec![vec![0i64; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let v = grid[i][j] as i64;
            if i == 0 && j == 0 {
                maxs[0][0] = v;
                mins[0][0] = v;
            } else if i == 0 {
                maxs[0][j] = maxs[0][j - 1] * v;
                mins[0][j] = mins[0][j - 1] * v;
            } else if j == 0 {
                maxs[i][0] = maxs[i - 1][0] * v;
                mins[i][0] = mins[i - 1][0] * v;
            } else {
                if v >= 0 {
                    maxs[i][j] = std::cmp::max(maxs[i - 1][j], maxs[i][j - 1]) * v;
                    mins[i][j] = std::cmp::min(mins[i - 1][j], mins[i][j - 1]) * v;
                } else {
                    maxs[i][j] = std::cmp::min(mins[i - 1][j], mins[i][j - 1]) * v;
                    mins[i][j] = std::cmp::max(maxs[i - 1][j], maxs[i][j - 1]) * v;
                }
            }
        }
    }

    if maxs[rows - 1][cols - 1] < 0 {
        -1
    } else {
        (maxs[rows - 1][cols - 1] % 1_000_000_007) as i32
    }
}

fn main() {
    let grid = vec![vec![1, -2, -1], vec![1, -2, 1], vec![3, -4, 1]];
    let ret = max_product_path(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]];
        let ret = max_product_path(grid);
        assert_eq!(ret, -1);
    }
    {
        let grid = vec![vec![1, -2, -1], vec![1, -2, 1], vec![3, -4, 1]];
        let ret = max_product_path(grid);
        assert_eq!(ret, 8);
    }
    {
        let grid = vec![vec![1, 3], vec![0, -4]];
        let ret = max_product_path(grid);
        assert_eq!(ret, 0);
    }
}
