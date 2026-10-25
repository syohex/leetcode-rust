fn closed_iskands(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    fn f(row: usize, col: usize, grid: &[Vec<i32>], visited: &mut [Vec<bool>]) -> bool {
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);

        let mut q = VecDeque::new();
        q.push_back((row as i32, col as i32));

        let mut is_closed = true;
        let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        while let Some((r, c)) = q.pop_front() {
            visited[r as usize][c as usize] = true;

            for (x, y) in &dirs {
                let nr = r + x;
                let nc = c + y;

                if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                    is_closed = false;
                    continue;
                }

                if grid[nr as usize][nc as usize] == 0 && !visited[nr as usize][nc as usize] {
                    q.push_back((nr, nc));
                }
            }
        }

        is_closed
    }

    let (rows, cols) = (grid.len(), grid[0].len());

    let mut ret = 0;
    let mut visited = vec![vec![false; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 && !visited[i][j] && f(i, j, &grid, &mut visited) {
                ret += 1;
            }
        }
    }

    ret
}
fn main() {
    let grid = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 1, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0],
    ];
    let ret = closed_iskands(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];
        let ret = closed_iskands(grid);
        assert_eq!(ret, 2);
    }
    {
        let grid = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
        ];
        let ret = closed_iskands(grid);
        assert_eq!(ret, 1);
    }
    {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];
        let ret = closed_iskands(grid);
        assert_eq!(ret, 2);
    }
}
