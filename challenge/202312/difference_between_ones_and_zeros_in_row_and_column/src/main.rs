fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut row_ones = vec![0; rows];
    let mut col_ones = vec![0; cols];
    for i in 0..rows {
        for j in 0..cols {
            row_ones[i] += grid[i][j];
            col_ones[j] += grid[i][j];
        }
    }

    let mut ret = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            ret[i][j] = row_ones[i] + col_ones[j]
                - (rows as i32 - row_ones[i])
                - (cols as i32 - col_ones[j]);
        }
    }
    ret
}

fn main() {
    let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
    let ret = ones_minus_zeros(grid);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
        let expected = vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]];
        let ret = ones_minus_zeros(grid);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
        let expected = vec![vec![5, 5, 5], vec![5, 5, 5]];
        let ret = ones_minus_zeros(grid);
        assert_eq!(ret, expected);
    }
}
