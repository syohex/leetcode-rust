fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut tmp = vec![];
    for row in &grid {
        for col in row {
            tmp.push(*col);
        }
    }

    let len = tmp.len();
    let k = k as usize % len;
    let base = (len - k) % len;
    let mut ret = vec![vec![0; cols]; rows];

    for i in 0..len {
        let (row, col) = (i / cols, i % cols);
        let idx = (base + i) % len;
        ret[row][col] = tmp[idx];
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ret = shift_grid(grid, 1);
    println!("ret={ret:?}");
}

#[test]
fn test() {
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
}
