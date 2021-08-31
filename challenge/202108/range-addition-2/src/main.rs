fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
    if ops.is_empty() {
        return m * n;
    }

    let mut min_row = std::i32::MAX;
    let mut min_col = std::i32::MAX;

    for op in ops {
        min_row = std::cmp::min(min_row, op[0]);
        min_col = std::cmp::min(min_col, op[1]);
    }

    min_row * min_col
}

fn main() {
    let ops = vec![vec![2, 2], vec![3, 3]];
    println!("{}", max_count(3, 3, ops));
}

#[test]
fn test_max_count() {
    {
        let ops = vec![vec![2, 2], vec![3, 3]];
        assert_eq!(max_count(3, 3, ops), 4);
    }
    {
        let ops = vec![
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
            vec![2, 2],
            vec![2, 2],
            vec![3, 3],
            vec![3, 3],
            vec![3, 3],
        ];
        assert_eq!(max_count(3, 3, ops), 4);
    }
    {
        let ops = vec![];
        assert_eq!(max_count(3, 3, ops), 9);
    }
}
