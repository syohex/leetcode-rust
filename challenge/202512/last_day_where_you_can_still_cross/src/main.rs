fn lastest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    fn f(rows: i32, cols: i32, day: usize, cells: &[Vec<i32>]) -> bool {
        let mut grid = vec![vec![0; cols as usize]; rows as usize];

        for i in 0..day {
            let (r, c) = (cells[i as usize][0], cells[i as usize][1]);
            grid[(r - 1) as usize][(c - 1) as usize] = 1;
        }

        let mut q = vec![];
        for i in 0..cols {
            if grid[0][i as usize] == 0 {
                q.push((0i32, i as i32));
                grid[0][i as usize] = 2;
            }
        }

        let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        while let Some((r, c)) = q.pop() {
            if r == rows - 1 {
                return true;
            }

            for &(x, y) in &steps {
                let row = r + x;
                let col = c + y;

                if row >= 0
                    && row < rows
                    && col >= 0
                    && col < cols
                    && grid[row as usize][col as usize] == 0
                {
                    q.push((row, col));
                    grid[row as usize][col as usize] = 2;
                }
            }
        }

        false
    }

    let mut left = 0;
    let mut right = row * col;

    while left < right {
        let mid = right - (right - left) / 2;
        if f(row, col, mid as usize, &cells) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

fn main() {
    let cells = vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]];
    let ret = lastest_day_to_cross(2, 2, cells);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let cells = vec![vec![1, 1], vec![2, 1], vec![1, 2], vec![2, 2]];
        let ret = lastest_day_to_cross(2, 2, cells);
        assert_eq!(ret, 2);
    }
    {
        let cells = vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]];
        let ret = lastest_day_to_cross(2, 2, cells);
        assert_eq!(ret, 1);
    }
    {
        let cells = vec![
            vec![1, 2],
            vec![2, 1],
            vec![3, 3],
            vec![2, 2],
            vec![1, 1],
            vec![1, 3],
            vec![2, 3],
            vec![3, 2],
            vec![3, 1],
        ];
        let ret = lastest_day_to_cross(3, 3, cells);
        assert_eq!(ret, 3);
    }
}
