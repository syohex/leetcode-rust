fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    if rows < 3 || cols < 3 {
        return 0;
    }

    let mut ret = 0;
    for i in 0..(rows - 2) {
        for j in 0..(cols - 2) {
            let mut checked = vec![false; 9];
            let mut ok = true;
            for x in 0..3 {
                for y in 0..3 {
                    let row = i + x;
                    let col = j + y;
                    if grid[row][col] >= 1
                        && grid[row][col] <= 9
                        && !checked[grid[row][col] as usize - 1]
                    {
                        checked[grid[row][col] as usize - 1] = true;
                    } else {
                        ok = false;
                        break;
                    }
                }
            }

            if !ok {
                continue;
            }

            let sum = grid[i][j] + grid[i][j + 1] + grid[i][j + 2];

            for k in 0..3 {
                let s = grid[i + k][j] + grid[i + k][j + 1] + grid[i + k][j + 2];
                if sum != s {
                    ok = false;
                    break;
                }

                let s = grid[i][j + k] + grid[i + 1][j + k] + grid[i + 2][j + k];
                if sum != s {
                    ok = false;
                    break;
                }
            }

            if !ok {
                continue;
            }

            let s = grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2];
            if sum != s {
                ok = false;
            } else {
                let s = grid[i][j + 2] + grid[i + 1][j + 1] + grid[i + 2][j];
                if sum != s {
                    ok = false;
                }
            }

            if ok {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![
        vec![4, 3, 8, 5, 5, 5],
        vec![9, 5, 1, 5, 5, 5],
        vec![2, 7, 6, 5, 5, 5],
    ];
    let ret = num_magic_squares_inside(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![4, 3, 8, 5, 5, 5],
            vec![9, 5, 1, 5, 5, 5],
            vec![2, 7, 6, 5, 5, 5],
        ];
        let ret = num_magic_squares_inside(grid);
        assert_eq!(ret, 1);
    }
    {
        let grid = vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]];
        let ret = num_magic_squares_inside(grid);
        assert_eq!(ret, 0);
    }
    {
        let grid = vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]];
        let ret = num_magic_squares_inside(grid);
        assert_eq!(ret, 1);
    }
    {
        let grid = vec![vec![8]];
        let ret = num_magic_squares_inside(grid);
        assert_eq!(ret, 0);
    }
}
