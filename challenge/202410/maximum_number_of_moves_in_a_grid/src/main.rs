fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut q: Vec<(usize, usize)> = (0..rows).into_iter().map(|i| (i, 0)).collect();

    let mut ret = 0;
    while !q.is_empty() {
        let (i, j) = q.pop().unwrap();
        ret = std::cmp::max(ret, j as i32);

        if j + 1 < cols {
            if i >= 1 && grid[i][j] < grid[i - 1][j + 1] {
                q.push((i - 1, j + 1));
            }
            if grid[i][j] < grid[i][j + 1] {
                q.push((i, j + 1));
            }
            if i + 1 < rows && grid[i][j] < grid[i + 1][j + 1] {
                q.push((i + 1, j + 1));
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![
        vec![2, 4, 3, 5],
        vec![5, 4, 9, 3],
        vec![3, 4, 2, 11],
        vec![10, 9, 13, 15],
    ];
    let ret = max_moves(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ];
        let ret = max_moves(grid);
        assert_eq!(ret, 3);
    }
    {
        let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
        let ret = max_moves(grid);
        assert_eq!(ret, 0);
    }
}
