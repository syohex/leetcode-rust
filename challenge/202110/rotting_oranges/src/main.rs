fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    let mut grid = grid;
    let mut freshes = 0;

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    for i in 0..rows {
        for j in 0..cols {
            let e = grid[i as usize][j as usize];
            if e == 1 {
                freshes += 1;
            } else if e == 2 {
                q.push_back((i, j))
            }
        }
    }

    q.push_back((-1, -1));

    let steps = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut minutes = -1;
    while !q.is_empty() {
        let (row, col) = q.pop_front().unwrap();
        if row == -1 && col == -1 {
            minutes += 1;
            if !q.is_empty() {
                q.push_back((-1, -1));
            }
        } else {
            for (i, j) in &steps {
                let x = row + *i;
                let y = col + *j;

                if x >= 0 && x < rows && y >= 0 && y < cols && grid[x as usize][y as usize] == 1 {
                    grid[x as usize][y as usize] = 2;
                    freshes -= 1;
                    q.push_back((x, y));
                }
            }
        }
    }

    if freshes > 0 {
        return -1;
    }

    minutes
}

fn main() {
    let grid: Vec<Vec<i32>> = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let ret = oranges_rotting(grid);
    println!("ret={}", ret);
}

#[test]
fn test_oranges_rotting() {
    {
        let grid: Vec<Vec<i32>> = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(oranges_rotting(grid), 4);
    }
    {
        let grid: Vec<Vec<i32>> = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(oranges_rotting(grid), -1);
    }
    {
        let grid: Vec<Vec<i32>> = vec![vec![0, 2]];
        assert_eq!(oranges_rotting(grid), 0);
    }
}
