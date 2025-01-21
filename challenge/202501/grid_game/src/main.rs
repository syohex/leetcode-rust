fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
    let mut upper_sum = grid[0].iter().fold(0i64, |acc, n| acc + *n as i64);
    let mut lower_sum = 0i64;

    let mut ret = i64::MAX;
    for i in 0..grid[0].len() {
        upper_sum -= grid[0][i] as i64;
        ret = std::cmp::min(ret, std::cmp::max(upper_sum, lower_sum));
        lower_sum += grid[1][i] as i64;
    }

    ret
}

fn main() {
    let grid = vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]];
    let ret = grid_game(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![2, 5, 4], vec![1, 5, 1]];
        let ret = grid_game(grid);
        assert_eq!(ret, 4);
    }
    {
        let grid = vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]];
        let ret = grid_game(grid);
        assert_eq!(ret, 7);
    }
    {
        let grid = vec![vec![3, 3, 1], vec![8, 5, 2]];
        let ret = grid_game(grid);
        assert_eq!(ret, 4);
    }
}
