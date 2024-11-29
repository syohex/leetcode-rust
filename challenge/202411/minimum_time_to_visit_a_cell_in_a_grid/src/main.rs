fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    if grid[0][1] > 1 && grid[1][0] > 1 {
        return -1;
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut min_times = vec![vec![std::i32::MAX; cols]; rows];
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, 0, 0)));

    let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while let Some(Reverse((time, row, col))) = q.pop() {
        if row == rows - 1 && col == cols - 1 {
            return time;
        }

        for &(x, y) in &steps {
            let (r, c) = (row as i32 + x, col as i32 + y);
            if r >= 0
                && r < rows as i32
                && c >= 0
                && c < cols as i32
            {
                let (r, c) = (r as usize, c as usize);
                let wait = if (grid[r][c] - time) % 2 == 0 { 1 } else { 0 };
                let next_time = std::cmp::max(time + 1, grid[r][c] + wait);

                if next_time < min_times[r][c] {
                    q.push(Reverse((next_time, r, c)));
                    min_times[r][c] = next_time;
                }
            }
        }
    }

    -1
}

fn main() {
    let grid = vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]];
    let ret = minimum_time(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]];
        let ret = minimum_time(grid);
        assert_eq!(ret, 7);
    }
    {
        let grid = vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]];
        let ret = minimum_time(grid);
        assert_eq!(ret, -1);
    }
}
