fn f(
    row: usize,
    col1: usize,
    col2: usize,
    grid: &Vec<Vec<i32>>,
    dp: &mut Vec<Vec<Vec<i32>>>,
) -> i32 {
    use std::cmp::max;

    if dp[row][col1][col2] != -1 {
        return dp[row][col1][col2];
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut ret = grid[row][col1];
    if col1 != col2 {
        ret += grid[row][col2];
    }

    if row < rows - 1 {
        let steps: Vec<i32> = vec![-1, 0, 1];
        let mut value = 0;
        for step1 in &steps {
            if (col1 == 0 && *step1 == -1) || (col1 == cols - 1 && *step1 == 1) {
                continue;
            }

            for step2 in &steps {
                if (col2 == 0 && *step2 == -1) || (col2 == cols - 1 && *step2 == 1) {
                    continue;
                }

                let new_col1 = (col1 as i32 + *step1) as usize;
                let new_col2 = (col2 as i32 + *step2) as usize;
                value = max(value, f(row + 1, new_col1, new_col2, grid, dp));
            }
        }

        ret += value;
    }

    dp[row][col1][col2] = ret;
    ret
}

fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![vec![-1; cols]; cols]; rows];
    f(0, 0, cols - 1, &grid, &mut dp)
}

fn main() {
    let grid = vec![vec![3, 1, 1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]];
    println!("ret={}", cherry_pickup(grid));
}

#[test]
fn test_cherry_pickup() {
    {
        let grid = vec![vec![3, 1, 1], vec![2, 5, 1], vec![1, 5, 5], vec![2, 1, 1]];
        assert_eq!(cherry_pickup(grid), 24);
    }
    {
        let grid = vec![
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![2, 0, 0, 0, 0, 3, 0],
            vec![2, 0, 9, 0, 0, 0, 0],
            vec![0, 3, 0, 5, 4, 0, 0],
            vec![1, 0, 2, 3, 0, 0, 6],
        ];
        assert_eq!(cherry_pickup(grid), 28);
    }
}
