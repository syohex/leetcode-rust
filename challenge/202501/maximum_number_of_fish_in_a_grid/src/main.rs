fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
    fn f(row: i32, col: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
        let mut ret = grid[row as usize][col as usize];

        grid[row as usize][col as usize] = 0;

        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        if row >= 1 && grid[(row - 1) as usize][col as usize] != 0 {
            ret += f(row - 1, col, grid);
        }
        if col + 1 < cols && grid[row as usize][(col + 1) as usize] != 0 {
            ret += f(row, col + 1, grid);
        }
        if row + 1 < rows && grid[(row + 1) as usize][col as usize] != 0 {
            ret += f(row + 1, col, grid);
        }
        if col >= 1 && grid[row as usize][(col - 1) as usize] != 0 {
            ret += f(row, col - 1, grid);
        }

        ret
    }

    let mut grid = grid;
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut ret = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != 0 {
                ret = std::cmp::max(ret, f(i as i32, j as i32, &mut grid));
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![
        vec![0, 2, 1, 0],
        vec![4, 0, 0, 3],
        vec![1, 0, 0, 4],
        vec![0, 3, 2, 0],
    ];
    let ret = find_max_fish(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![0, 2, 1, 0],
            vec![4, 0, 0, 3],
            vec![1, 0, 0, 4],
            vec![0, 3, 2, 0],
        ];
        let ret = find_max_fish(grid);
        assert_eq!(ret, 7);
    }
    {
        let grid = vec![
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
        ];
        let ret = find_max_fish(grid);
        assert_eq!(ret, 1);
    }
}
