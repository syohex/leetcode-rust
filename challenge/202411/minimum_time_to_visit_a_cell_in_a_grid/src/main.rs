fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    if grid[0][1] > 1 && grid[1][0] > 1 {
        return -1;
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), (0, 0)));

    let mut visited = vec![vec![false; cols]; rows];
    let steps = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    while let Some((Reverse(t), (row, col))) = q.pop() {
        if row == rows - 1 && col == cols - 1 {
            return t;
        }

        visited[row][col] = true;

        for &(x, y) in &steps {
            let (r, c) = (row as i32 + x, col as i32 + y);
            if r >= 0
                && r < rows as i32
                && c >= 0
                && c < cols as i32
                && !visited[r as usize][c as usize]
            {
                let (r, c) = (r as usize, c as usize);
                let diff = grid[r][c] - t;
                if diff > 1 {
                    let diff = if diff % 2 == 0 { 1 } else { 0 };
                    q.push((Reverse(grid[r][c] + diff), (r, c)))
                } else {
                    q.push((Reverse(t + 1), (r, c)));
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
