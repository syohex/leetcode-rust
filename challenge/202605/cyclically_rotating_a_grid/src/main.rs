fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut grid = grid;
    let (rows, cols) = (grid.len(), grid[0].len());
    let layers = std::cmp::min(rows / 2, cols / 2);
    let k = k as usize;

    for layer in 0..layers {
        let mut indexes = vec![];
        let mut vals = vec![];

        // left side
        for row in layer..(rows - layer - 1) {
            indexes.push((row, layer));
            vals.push(grid[row][layer]);
        }

        // bottom side
        for col in layer..(cols - layer - 1) {
            indexes.push((rows - 1 - layer, col));
            vals.push(grid[rows - 1 - layer][col]);
        }

        // right side
        for row in ((layer+1)..=(rows - 1 - layer)).rev() {
            indexes.push((row, cols - 1 - layer));
            vals.push(grid[row][cols - 1 - layer]);
        }

        // upper side
        for col in ((layer+1)..=(cols - 1 - layer)).rev() {
            indexes.push((layer, col));
            vals.push(grid[layer][col]);
        }

        let len = vals.len();
        let k = k % len;
        for i in 0..len {
            let pos = (i + len - k) % len;
            grid[indexes[i].0][indexes[i].1] = vals[pos];
        }
    }

    grid
}

fn main() {
    let grid = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    let ret = rotate_grid(grid, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![40, 10], vec![30, 20]];
        let expected = vec![vec![10, 20], vec![40, 30]];
        let ret = rotate_grid(grid, 1);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let expected = vec![
            vec![3, 4, 8, 12],
            vec![2, 11, 10, 16],
            vec![1, 7, 6, 15],
            vec![5, 9, 13, 14],
        ];
        let ret = rotate_grid(grid, 2);
        assert_eq!(ret, expected);
    }
}
