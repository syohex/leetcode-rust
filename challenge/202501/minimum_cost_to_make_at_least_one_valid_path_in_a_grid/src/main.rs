fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let (rows, cols) = (grid.len(), grid[0].len());
    let steps = [(0, 1, 1), (0, -1, 2), (1, 0, 3), (-1, 0, 4)];
    let mut min_cost = vec![vec![std::i32::MAX; cols]; rows];

    let mut q = BinaryHeap::new();
    q.push((Reverse(0), (0, 0)));
    min_cost[0][0] = 0;

    while let Some((Reverse(cost), (row, col))) = q.pop() {
        let sign = grid[row as usize][col as usize];

        for &(x, y, s) in &steps {
            let (r, c) = (row + x, col + y);
            if r >= 0 && r < rows as i32 && c >= 0 && c < cols as i32 {
                let new_cost = cost + if sign == s { 0 } else { 1 };
                if new_cost < min_cost[r as usize][c as usize] {
                    min_cost[r as usize][c as usize] = new_cost;
                    q.push((Reverse(new_cost), (r, c)));
                }
            }
        }
    }

    min_cost[rows - 1][cols - 1]
}

fn main() {
    let grid = vec![
        vec![1, 1, 1, 1],
        vec![2, 2, 2, 2],
        vec![1, 1, 1, 1],
        vec![2, 2, 2, 2],
    ];
    let ret = min_cost(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
        ];
        let ret = min_cost(grid);
        assert_eq!(ret, 3);
    }
    {
        let grid = vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]];
        let ret = min_cost(grid);
        assert_eq!(ret, 0);
    }
    {
        let grid = vec![vec![1, 2], vec![4, 3]];
        let ret = min_cost(grid);
        assert_eq!(ret, 1);
    }
    {
        let grid = vec![vec![4]];
        let ret = min_cost(grid);
        assert_eq!(ret, 0);
    }
}
