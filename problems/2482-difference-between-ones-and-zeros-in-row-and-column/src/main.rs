fn one_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut row_sum = vec![];
    let mut col_sum = vec![];
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        let mut sum = 0;
        for j in 0..cols {
            sum += grid[i][j];
        }
        row_sum.push(sum);
    }
    for i in 0..cols {
        let mut sum = 0;
        for j in 0..rows {
            sum += grid[j][i];
        }
        col_sum.push(sum);
    }

    let mut ret = vec![vec![0; cols]; rows];
    let size = (rows + cols) as i32;
    for i in 0..rows {
        for j in 0..cols {
            ret[i][j] = size - ((size - (row_sum[i] + col_sum[j])) * 2);
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
    let ret = one_minus_zeros(grid);
    println!("ret={:?}", ret);
}

#[test]
fn test_one_minus_zeros() {
    {
        let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
        let expected = vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]];
        let ret = one_minus_zeros(grid);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
        let expected = vec![vec![5, 5, 5], vec![5, 5, 5]];
        let ret = one_minus_zeros(grid);
        assert_eq!(ret, expected);
    }
}
