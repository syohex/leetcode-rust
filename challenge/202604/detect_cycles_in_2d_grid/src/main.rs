fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    fn f(
        row: i32,
        col: i32,
        parent: (i32, i32),
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        visited[row as usize][col as usize] = true;

        let ch = grid[row as usize][col as usize];

        for (x, y) in steps {
            let (r, c) = (row + x, col + y);
            if r >= 0 && r < rows && c >= 0 && c < cols && ch == grid[r as usize][c as usize] {
                if !visited[r as usize][c as usize] {
                    if f(r, c, (row, col), grid, visited) {
                        return true;
                    }
                } else if (r, c) != parent {
                    return true;
                }
            }
        }

        false
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            let (row, col) = (i as i32, j as i32);
            if !visited[i][j] && f(row, col, (-1, -1), &grid, &mut visited) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let grid = vec![
        vec!['a', 'a', 'a', 'a'],
        vec!['a', 'b', 'b', 'a'],
        vec!['a', 'b', 'b', 'a'],
        vec!['a', 'a', 'a', 'a'],
    ];
    let ret = contains_cycle(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec!['c', 'a', 'd'],
            vec!['a', 'a', 'a'],
            vec!['a', 'a', 'd'],
            vec!['a', 'c', 'd'],
            vec!['a', 'b', 'c'],
        ];
        let ret = contains_cycle(grid);
        assert!(ret);
    }
    {
        let grid = vec![
            vec!['f', 'a', 'a', 'c', 'b'],
            vec!['e', 'a', 'a', 'e', 'c'],
            vec!['c', 'f', 'b', 'b', 'b'],
            vec!['c', 'e', 'a', 'b', 'e'],
            vec!['f', 'e', 'f', 'b', 'f'],
        ];
        let ret = contains_cycle(grid);
        assert!(ret);
    }
    {
        let grid = vec![
            vec!['a', 'b', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a'],
        ];
        let ret = contains_cycle(grid);
        assert!(!ret);
    }
    {
        let grid = vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a'],
        ];
        let ret = contains_cycle(grid);
        assert!(ret);
    }
    {
        let grid = vec![
            vec!['c', 'c', 'c', 'a'],
            vec!['c', 'd', 'c', 'c'],
            vec!['c', 'c', 'e', 'c'],
            vec!['f', 'c', 'c', 'c'],
        ];
        let ret = contains_cycle(grid);
        assert!(ret);
    }
}
