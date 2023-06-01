fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;

    let n = grid.len() as i32;
    let steps = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut grid = grid;
    let mut q = VecDeque::new();
    if grid[0][0] == 0 {
        q.push_back((0, 0));
        grid[0][0] = 1;
    }

    let mut count = 1;
    while !q.is_empty() {
        let len = q.len();
        for _ in 0..len {
            if let Some((x, y)) = q.pop_front() {
                if x == n - 1 && y == n - 1 {
                    return count;
                }

                for (a, b) in &steps {
                    let row = x + a;
                    let col = y + b;

                    if row >= 0
                        && row < n
                        && col >= 0
                        && col < n
                        && grid[row as usize][col as usize] == 0
                    {
                        q.push_back((row, col));
                        grid[row as usize][col as usize] = 1;
                    }
                }
            }
        }

        count += 1;
    }

    -1
}

fn main() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    let ret = shortest_path_binary_matrix(grid);
    println!("ret={ret}");
}

#[test]
fn test_shortest_path_binary_matrix() {
    {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let ret = shortest_path_binary_matrix(grid);
        assert_eq!(ret, 2);
    }
    {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let ret = shortest_path_binary_matrix(grid);
        assert_eq!(ret, 4);
    }
    {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let ret = shortest_path_binary_matrix(grid);
        assert_eq!(ret, -1);
    }

    {
        let grid = vec![vec![1]];
        let ret = shortest_path_binary_matrix(grid);
        assert_eq!(ret, -1);
    }
    {
        let grid = vec![vec![0]];
        let ret = shortest_path_binary_matrix(grid);
        assert_eq!(ret, 1);
    }
}
