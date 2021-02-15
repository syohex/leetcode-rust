fn count_island(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    grid[i][j] = 0;

    let mut count = 1;
    if i >= 1 && grid[i - 1][j] == 1 {
        count += count_island(grid, i - 1, j);
    }
    if i + 1 < grid.len() && grid[i + 1][j] == 1 {
        count += count_island(grid, i + 1, j);
    }
    if j >= 1 && grid[i][j - 1] == 1 {
        count += count_island(grid, i, j - 1);
    }
    if j + 1 < grid[i].len() && grid[i][j + 1] == 1 {
        count += count_island(grid, i, j + 1);
    }

    count
}

fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid.clone();
    let mut ret = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 1 {
                ret = std::cmp::max(ret, count_island(&mut grid, i, j));
            }
        }
    }

    ret
}

fn main() {
    let grid = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];
    println!("answer={}", max_area_of_island(grid));
}

#[test]
fn test_max_area_of_island() {
    {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(max_area_of_island(grid), 6);
    }
    {
        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(max_area_of_island(grid), 0);
    }
}
