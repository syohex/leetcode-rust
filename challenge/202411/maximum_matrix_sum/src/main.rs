fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let (sum, negatives, min) =
        matrix
            .into_iter()
            .fold((0i64, 0, std::i32::MAX), |(acc, negatives, min), v| {
                v.into_iter()
                    .fold((acc, negatives, min), |(acc, negatives, min), n| {
                        (
                            acc + n.abs() as i64,
                            if n >= 0 { negatives } else { negatives + 1 },
                            std::cmp::min(min, n.abs()),
                        )
                    })
            });

    if negatives % 2 != 0 {
        sum - 2 * min as i64
    } else {
        sum
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
    let ret = max_matrix_sum(matrix);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let matrix = vec![vec![1, -1], vec![-1, 1]];
        let ret = max_matrix_sum(matrix);
        assert_eq!(ret, 4);
    }
    {
        let matrix = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
        let ret = max_matrix_sum(matrix);
        assert_eq!(ret, 16);
    }
    {
        let matrix = vec![vec![-1, 0, -1], vec![-2, 1, 3], vec![3, 2, 2]];
        let ret = max_matrix_sum(matrix);
        assert_eq!(ret, 15);
    }
}
