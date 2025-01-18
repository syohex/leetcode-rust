fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; cols]; rows];

    let mut q = BinaryHeap::new();
    q.push((Reverse(0), (0, 0)));

    while let Some((Reverse(cost), (row, col))) = q.pop() {
        if visited[row][col] {
            continue;
        }
        visited[row][col] = true;

        if row == rows - 1 && col == cols - 1 {
            return cost;
        }

        let sign = grid[row][col];
        if sign == 1 && col + 1 < cols {
            q.push((Reverse(cost), (row, col + 1)));
        } else if sign == 2 && col >= 1 {
            q.push((Reverse(cost), (row, col - 1)));
        } else if sign == 3 && row + 1 < rows {
            q.push((Reverse(cost), (row + 1, col)));
        } else if sign == 4 && row >= 1 {
            q.push((Reverse(cost), (row - 1, col)));
        }

        if sign != 1 && col + 1 < cols {
            q.push((Reverse(cost + 1), (row, col + 1)));
        }
        if sign != 2 && col >= 1 {
            q.push((Reverse(cost + 1), (row, col - 1)));
        }
        if sign != 3 && row + 1 < rows {
            q.push((Reverse(cost + 1), (row + 1, col)));
        }
        if sign != 4 && row >= 1 {
            q.push((Reverse(cost + 1), (row - 1, col)));
        }
    }

    unreachable!("never reach here");
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
}
