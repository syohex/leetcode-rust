fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let mut ret = 0;
    let (rows, cols) = (grid.len(), grid[0].len());

    for j in 0..cols {
        let mut prev = grid[0][j];
        for i in 1..rows {
            if prev >= grid[i][j] {
                ret += prev - grid[i][j] + 1;
                prev += 1;
            } else {
                prev = grid[i][j];
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![3, 2], vec![1, 3], vec![3, 4], vec![0, 1]];
    let ret = minimum_operations(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![3, 2], vec![1, 3], vec![3, 4], vec![0, 1]];
        let ret = minimum_operations(grid);
        assert_eq!(ret, 15);
    }
    {
        let grid = vec![vec![3, 2, 1], vec![2, 1, 0], vec![1, 2, 3]];
        let ret = minimum_operations(grid);
        assert_eq!(ret, 12);
    }
}
