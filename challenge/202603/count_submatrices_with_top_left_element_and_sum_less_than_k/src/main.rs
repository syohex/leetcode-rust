fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut acc = vec![vec![0; cols + 1]; rows + 1];

    let mut ret = 0;
    for i in 0..rows {
        for j in 0..cols {
            acc[i + 1][j + 1] = grid[i][j] + acc[i][j + 1] + acc[i + 1][j] - acc[i][j];
            if acc[i + 1][j + 1] > k {
                break;
            }

            ret += 1;
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
    let ret = count_submatrices(grid, 20);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![7, 6, 3], vec![6, 6, 1]];
        let ret = count_submatrices(grid, 18);
        assert_eq!(ret, 4);
    }
    {
        let grid = vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]];
        let ret = count_submatrices(grid, 20);
        assert_eq!(ret, 6);
    }
}
