fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();

    grid.into_iter()
        .map(|v| v.iter().filter(|&n| *n == 1).count())
        .position(|ones| ones == n - 1)
        .unwrap() as i32
}

fn main() {
    let grid = vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]];
    let ret = find_champion(grid);
    println!("ret={ret}");
}

#[test]
fn test_find_champion() {
    {
        let grid = vec![vec![0, 1], vec![0, 0]];
        let ret = find_champion(grid);
        assert_eq!(ret, 0);
    }
    {
        let grid = vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]];
        let ret = find_champion(grid);
        assert_eq!(ret, 1);
    }
}
