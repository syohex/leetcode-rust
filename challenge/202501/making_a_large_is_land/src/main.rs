fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashMap, HashSet};

    fn set_color(row: usize, col: usize, color: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
        let mut ret = 1;
        grid[row][col] = color;

        let (rows, cols) = (grid.len(), grid[0].len());
        if row >= 1 && grid[row - 1][col] == 1 {
            ret += set_color(row - 1, col, color, grid);
        }
        if row + 1 < rows && grid[row + 1][col] == 1 {
            ret += set_color(row + 1, col, color, grid);
        }
        if col >= 1 && grid[row][col - 1] == 1 {
            ret += set_color(row, col - 1, color, grid);
        }
        if col + 1 < cols && grid[row][col + 1] == 1 {
            ret += set_color(row, col + 1, color, grid);
        }

        ret
    }

    let mut grid = grid;
    let mut island_size = HashMap::new();
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut color = 2;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                island_size.insert(color, set_color(i, j, color, &mut grid));
                color += 1;
            }
        }
    }

    if color == 2 {
        return 1;
    }
    if color == 3 {
        if let Some(&v) = island_size.get(&2) {
            if v as usize == rows * cols {
                return v;
            }

            return v + 1;
        }
    }

    let mut ret = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                let mut size = 1;
                let mut colors = HashSet::new();
                if i >= 1 && grid[i - 1][j] >= 2 {
                    colors.insert(grid[i - 1][j]);
                }
                if i + 1 < rows && grid[i + 1][j] >= 2 {
                    colors.insert(grid[i + 1][j]);
                }
                if j >= 1 && grid[i][j - 1] >= 2 {
                    colors.insert(grid[i][j - 1]);
                }
                if j + 1 < cols && grid[i][j + 1] >= 2 {
                    colors.insert(grid[i][j + 1]);
                }

                for color in &colors {
                    size += *island_size.get(color).unwrap();
                }

                ret = std::cmp::max(ret, size);
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 0], vec![0, 1]];
    let ret = largest_island(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1, 0], vec![0, 1]];
        let ret = largest_island(grid);
        assert_eq!(ret, 3);
    }
    {
        let grid = vec![vec![1, 1], vec![1, 0]];
        let ret = largest_island(grid);
        assert_eq!(ret, 4);
    }
    {
        let grid = vec![vec![1, 1], vec![1, 1]];
        let ret = largest_island(grid);
        assert_eq!(ret, 4);
    }
    {
        let grid = vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 1, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 0, 0],
            vec![1, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 0, 0],
            vec![0, 1, 1, 1, 1, 0, 0],
        ];
        let ret = largest_island(grid);
        assert_eq!(ret, 18);
    }
}
