fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut ret = vec![vec![0; cols]; rows];

    let modulo = 12345i64;
    let mut suffix = 1i64;
    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            ret[i][j] = suffix as i32;
            suffix = suffix * grid[i][j] as i64 % modulo;
        }
    }

    let mut prefix = 1i64;
    for i in 0..rows {
        for j in 0..cols {
            ret[i][j] = (ret[i][j] as i64 * prefix % modulo) as i32;
            prefix = prefix * grid[i][j] as i64 % modulo;
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 2], vec![3, 4]];
    let ret = construct_product_matrix(grid);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![414750857], vec![449145368], vec![767292749]];
        let expected = vec![vec![1462], vec![3103], vec![9436]];
        let ret = construct_product_matrix(grid);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![24, 12], vec![8, 6]];
        let ret = construct_product_matrix(grid);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![12345], vec![2], vec![1]];
        let expected = vec![vec![2], vec![0], vec![0]];
        let ret = construct_product_matrix(grid);
        assert_eq!(ret, expected);
    }
}
