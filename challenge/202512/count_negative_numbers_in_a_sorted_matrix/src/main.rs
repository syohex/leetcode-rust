fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    grid.into_iter()
        .fold(0, |acc, v| acc + v.len() - (v.partition_point(|&n| n >= 0))) as i32
}

fn main() {
    let grid = vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
    ];
    let ret = count_negatives(grid);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let grid = vec![
            vec![4, 3, 2, -1],
            vec![3, 2, 1, -1],
            vec![1, 1, -1, -2],
            vec![-1, -1, -2, -3],
        ];
        let ret = count_negatives(grid);
        assert_eq!(ret, 8);
    }
    {
        let grid = vec![vec![3, 2], vec![1, 0]];
        let ret = count_negatives(grid);
        assert_eq!(ret, 0);
    }
}
