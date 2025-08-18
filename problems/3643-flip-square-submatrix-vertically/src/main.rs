fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
    let mut ret = grid.clone();
    let x = x as usize;
    let y = y as usize;
    let k = k as usize;

    for i in 0..k {
        for j in 0..k {
            ret[x + i][y + j] = grid[x + k - 1 - i][y + j];
        }
    }

    ret
}

fn main() {
    let grid = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    let ret = reverse_submatrix(grid, 1, 0, 3);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let expected = vec![
            vec![1, 2, 3, 4],
            vec![13, 14, 15, 8],
            vec![9, 10, 11, 12],
            vec![5, 6, 7, 16],
        ];
        let ret = reverse_submatrix(grid, 1, 0, 3);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]];
        let expected = vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3]];
        let ret = reverse_submatrix(grid, 0, 2, 2);
        assert_eq!(ret, expected);
    }
}
