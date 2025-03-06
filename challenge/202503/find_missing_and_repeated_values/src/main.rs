fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let len = grid.len();
    grid.into_iter()
        .fold(vec![0; len * len], |mut acc, v| {
            for n in v {
                acc[(n - 1) as usize] += 1;
            }
            acc
        })
        .into_iter()
        .enumerate()
        .fold(vec![0; 2], |mut acc, (i, n)| {
            match n {
                0 => acc[1] = (i + 1) as i32,
                2 => acc[0] = (i + 1) as i32,
                _ => (),
            }
            acc
        })
}

fn main() {
    let grid = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
    let ret = find_missing_and_repeated_values(grid);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let grid = vec![vec![1, 3], vec![2, 2]];
        let expected = vec![2, 4];
        let ret = find_missing_and_repeated_values(grid);
        assert_eq!(ret, expected);
    }
    {
        let grid = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
        let expected = vec![9, 5];
        let ret = find_missing_and_repeated_values(grid);
        assert_eq!(ret, expected);
    }
}
