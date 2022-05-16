fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut grid = grid;
    if grid[0][0] == 0 {
        q.push_back((0, 0));
        grid[0][0] = 1;
    }

    let row_max = (grid.len() - 1) as i32;
    let col_max = (grid[0].len() - 1) as i32;

    let steps = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    while !q.is_empty() {
        let (row, col) = q.front().unwrap().clone();
        q.pop_front();

        if row == row_max && col == col_max {
            return grid[row as usize][col as usize];
        }

        for (x, y) in &steps {
            let r = row + *x;
            let c = col + *y;

            if r >= 0 && r <= row_max && c >= 0 && c <= col_max && grid[r as usize][c as usize] == 0
            {
                grid[r as usize][c as usize] = grid[row as usize][col as usize] + 1;
                q.push_back((r, c));
            }
        }
    }

    -1
}

fn main() {
    let grid = vec![
        vec![0, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 1, 1],
        vec![0, 1, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 0, 1],
        vec![0, 0, 1, 0, 1, 0],
    ];
    let ret = shortest_path_binary_matrix(grid);
    println!("ret={ret}");
}

#[test]
fn test_shortest_binary_matrix() {
    {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(shortest_path_binary_matrix(grid), 2);
    }
    {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(shortest_path_binary_matrix(grid), 4);
    }
    {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(shortest_path_binary_matrix(grid), -1);
    }
    {
        let grid = vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 1, 0, 0, 0, 1],
            vec![1, 0, 0, 1, 0, 1],
            vec![0, 0, 1, 0, 1, 0],
        ];
        assert_eq!(shortest_path_binary_matrix(grid), 7);
    }
}
