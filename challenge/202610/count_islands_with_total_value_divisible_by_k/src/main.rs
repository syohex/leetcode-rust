fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    fn f(row: i32, col: i32, grid: &[Vec<i32>], visited: &mut [Vec<bool>]) -> i64 {
        use std::collections::VecDeque;

        visited[row as usize][col as usize] = true;
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut q = VecDeque::new();
        q.push_back((row, col));

        let mut sum = 0i64;
        let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        while let Some((row, col)) = q.pop_front() {
            sum += grid[row as usize][col as usize] as i64;

            for (x, y) in &dirs {
                let r = row + x;
                let c = col + y;

                if r >= 0
                    && r < rows
                    && c >= 0
                    && c < cols
                    && grid[r as usize][c as usize] != 0
                    && !visited[r as usize][c as usize]
                {
                    visited[r as usize][c as usize] = true;
                    q.push_back((r, c));
                }
            }
        }

        sum
    }

    let k = k as i64;
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; cols]; rows];
    let mut ret = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] != 0
                && !visited[i][j]
                && f(i as i32, j as i32, &grid, &mut visited) % k == 0
            {
                ret += 1;
            }
        }
    }
    ret
}

fn main() {
    let grid = vec![
        vec![0, 2, 1, 0, 0],
        vec![0, 5, 0, 0, 5],
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 4, 7, 0],
        vec![0, 2, 0, 0, 8],
    ];
    let ret = count_islands(grid, 5);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![0, 0, 1, 0],
            vec![18, 2, 20, 20],
            vec![0, 4, 5, 0],
            vec![0, 0, 0, 0],
        ];
        let ret = count_islands(grid, 5);
        assert_eq!(ret, 1);
    }
    {
        let grid = vec![
            vec![0, 2, 1, 0, 0],
            vec![0, 5, 0, 0, 5],
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 4, 7, 0],
            vec![0, 2, 0, 0, 8],
        ];
        let ret = count_islands(grid, 5);
        assert_eq!(ret, 2);
    }
    {
        let grid = vec![vec![3, 0, 3, 0], vec![0, 3, 0, 3], vec![3, 0, 3, 0]];
        let ret = count_islands(grid, 3);
        assert_eq!(ret, 6);
    }
}
