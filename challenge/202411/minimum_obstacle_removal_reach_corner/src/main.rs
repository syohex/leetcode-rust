fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let (rows, cols) = (grid.len(), grid[0].len());
    let moves = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut min_steps = vec![vec![i32::MAX; cols]; rows];

    let mut q = BinaryHeap::new();
    q.push((Reverse(0), (0, 0)));

    while let Some((Reverse(steps), (row, col))) = q.pop() {
        for &(x, y) in &moves {
            let (r, c) = (row + x, col + y);
            if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
                let (r, c) = (r as usize, c as usize);
                let steps = steps + grid[r][c];
                if steps < min_steps[r][c] {
                    min_steps[r][c] = steps;
                    q.push((Reverse(steps), (r as i32, c as i32)));
                }
            }
        }
    }

    min_steps[rows - 1][cols - 1]
}

fn main() {
    let grid = vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 0, 0, 1, 0],
    ];
    let ret = minimum_obstacles(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![0, 1, 1], vec![1, 1, 0], vec![1, 1, 0]];
        let ret = minimum_obstacles(grid);
        assert_eq!(ret, 2);
    }
    {
        let grid = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0],
        ];
        let ret = minimum_obstacles(grid);
        assert_eq!(ret, 0);
    }
}
