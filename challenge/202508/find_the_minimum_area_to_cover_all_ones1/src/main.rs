fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut min_row = rows;
    let mut min_col = cols;
    let mut max_row = 0;
    let mut max_col = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                min_row = std::cmp::min(min_row, i);
                min_col = std::cmp::min(min_col, j);
                max_row = std::cmp::max(max_row, i);
                max_col = std::cmp::max(max_col, j);
            }
        }
    }

    ((max_row - min_row + 1) * (max_col - min_col + 1)) as i32
}

fn main() {
    let grid = vec![vec![0, 1, 0], vec![1, 0, 1]];
    let ret = minimum_area(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![0, 1, 0], vec![1, 0, 1]];
        let ret = minimum_area(grid);
        assert_eq!(ret, 6);
    }
    {
        let grid = vec![vec![1, 0], vec![0, 0]];
        let ret = minimum_area(grid);
        assert_eq!(ret, 1);
    }
}
