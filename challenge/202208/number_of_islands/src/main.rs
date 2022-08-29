fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    fn f(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
        grid[row][col] = '0';

        if row >= 1 && grid[row - 1][col] == '1' {
            f(grid, row - 1, col);
        }
        if row + 1 < grid.len() && grid[row + 1][col] == '1' {
            f(grid, row + 1, col);
        }
        if col >= 1 && grid[row][col - 1] == '1' {
            f(grid, row, col - 1);
        }
        if col + 1 < grid[row].len() && grid[row][col + 1] == '1' {
            f(grid, row, col + 1);
        }
    }

    let mut grid = grid;
    let mut ret = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == '1' {
                f(&mut grid, i, j);
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let ret = num_islands(grid);
    println!("ret={ret}");
}

#[test]
fn test_num_islands() {
    {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let ret = num_islands(grid);
        assert_eq!(ret, 1);
    }
    {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let ret = num_islands(grid);
        assert_eq!(ret, 3);
    }
}
