fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;

    let rows = grid.len();
    let cols = grid[0].len();

    let mut start = (0, 0);
    let mut len = 0usize;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                start = (i as i32, j as i32);
            }

            if grid[i][j] != -1 {
                len += 1;
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut q = vec![];
    q.push((start, visited));

    let steps = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

    let mut ret = 0;
    while !q.is_empty() {
        let ((r, c), visited) = q.pop().unwrap();

        if grid[r as usize][c as usize] == 2 {
            if visited.len() == len {
                ret += 1;
            }
            continue;
        }

        for (i, j) in &steps {
            let row = r + i;
            let col = c + j;

            if row >= 0
                && (row as usize) < rows
                && col >= 0
                && (col as usize) < cols
                && grid[row as usize][col as usize] != -1
                && !visited.contains(&(row, col))
            {
                let mut v = visited.clone();
                v.insert((row, col));
                q.push(((row, col), v));
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
    let ret = unique_paths_iii(grid);
    println!("ret={ret}");
}

#[test]
fn test_unique_paths_iii() {
    {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 2, -1]];
        let ret = unique_paths_iii(grid);
        assert_eq!(ret, 2);
    }
    {
        let grid = vec![vec![1, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 2]];
        let ret = unique_paths_iii(grid);
        assert_eq!(ret, 4);
    }
    {
        let grid = vec![vec![0, 1], vec![2, 0]];
        let ret = unique_paths_iii(grid);
        assert_eq!(ret, 0);
    }
}
