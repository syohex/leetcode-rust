fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let (rows, cols) = (matrix.len(), matrix[0].len());

    let mut dp = vec![vec![None; cols]; rows];
    for i in 0..cols {
        dp[0][i] = Some(matrix[0][i]);
    }

    for i in 1..rows {
        for j in 0..cols {
            let steps: &[i32] = if j == 0 {
                &[0, 1]
            } else if j == cols - 1 {
                &[-1, 0]
            } else {
                &[-1, 0, 1]
            };

            for k in steps {
                let sum = dp[i - 1][(j as i32 + k) as usize].unwrap() + matrix[i][j];
                if let Some(v) = dp[i][j] {
                    dp[i][j] = Some(std::cmp::min(v, sum));
                } else {
                    dp[i][j] = Some(sum);
                }
            }
        }
    }

    let mut ret = std::i32::MAX;
    for v in dp[rows - 1].iter() {
        if let Some(n) = v {
            ret = std::cmp::min(ret, *n);
        }
    }

    ret
}

fn main() {
    let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    let ret = min_falling_path_sum(matrix);
    println!("ret={ret}");
}

#[test]
fn test_min_falling_path_sum() {
    {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        let ret = min_falling_path_sum(matrix);
        assert_eq!(ret, 13);
    }
    {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        let ret = min_falling_path_sum(matrix);
        assert_eq!(ret, -59);
    }
}
