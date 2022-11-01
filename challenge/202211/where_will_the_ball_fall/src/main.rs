fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret = vec![];

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..cols {
        let mut row = 0;
        let mut col = i;
        let mut ok = true;
        while row < rows {
            if grid[row][col] == 1 {
                if col == cols - 1 || grid[row][col + 1] == -1 {
                    ok = false;
                    break;
                }

                col += 1;
            } else {
                if col == 0 || grid[row][col - 1] == 1 {
                    ok = false;
                    break;
                }

                col -= 1;
            }

            row += 1;
        }

        if ok {
            ret.push(col as i32);
        } else {
            ret.push(-1);
        }
    }

    ret
}

fn main() {
    let grid = vec![
        vec![1, 1, 1, 1, 1, 1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![1, 1, 1, 1, 1, 1],
        vec![-1, -1, -1, -1, -1, -1],
    ];
    let ret = find_ball(grid);
    println!("ret={:?}", ret);
}

#[test]
fn test_find_ball() {
    {
        let grid = vec![
            vec![1, 1, 1, -1, -1],
            vec![1, 1, 1, -1, -1],
            vec![-1, -1, -1, 1, 1],
            vec![1, 1, 1, 1, -1],
            vec![-1, -1, -1, -1, -1],
        ];
        let expected = vec![1, -1, -1, -1, -1];
        let ret = find_ball(grid);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![-1]];
        let expected = vec![-1];
        let ret = find_ball(grid);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
        ];
        let expected = vec![0, 1, 2, 3, 4, -1];
        let ret = find_ball(grid);
        assert_eq!(ret, expected);
    }
}
