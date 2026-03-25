fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut row_sum = vec![0i64; rows];
    let mut col_sum = vec![0i64; cols];
    let mut total_sum = 0i64;

    for i in 0..rows {
        for j in 0..cols {
            let v = grid[i][j] as i64;
            row_sum[i] += v;
            col_sum[j] += v;
            total_sum += v;
        }
    }

    let mut bottom_sum = row_sum[0];
    for v in row_sum.into_iter().skip(1) {
        if bottom_sum == total_sum - bottom_sum {
            return true;
        }

        bottom_sum += v;
    }

    let mut left_sum = col_sum[0];
    for v in col_sum.into_iter().skip(1) {
        if left_sum == total_sum - left_sum {
            return true;
        }

        left_sum += v;
    }

    false
}

fn main() {
    let grid = vec![vec![1, 4], vec![2, 3]];
    let ret = can_partition_grid(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1, 4], vec![2, 3]];
        let ret = can_partition_grid(grid);
        assert!(ret);
    }
    {
        let grid = vec![vec![1, 3], vec![2, 4]];
        let ret = can_partition_grid(grid);
        assert!(!ret);
    }
}
