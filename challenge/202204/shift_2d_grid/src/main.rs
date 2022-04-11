fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let cells = rows * cols;

    let mut ret = vec![vec![0; cols]; rows];
    let k = k as usize % cells;
    let base = cells - k;

    for i in 0..cells {
        let row = i / cols;
        let col = i % cols;

        let grid_index = (base + i) % cells;
        let r = grid_index / cols;
        let c = grid_index % cols;

        ret[row][col] = grid[r][c];
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ret = shift_grid(grid, 1);
    println!("ret={:?}", ret);
}

#[test]
fn test_shift_grid() {
    {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let ret = shift_grid(grid, 1);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ];
        let expected = vec![
            vec![12, 0, 21, 13],
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
        ];
        let ret = shift_grid(grid, 4);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let ret = shift_grid(grid, 9);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![7],
            vec![6],
            vec![5],
        ];
        let expected = vec![
            vec![6],
            vec![5],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![7],
        ];
        let ret = shift_grid(grid, 23);
        assert_eq!(ret, expected);
    }
}
