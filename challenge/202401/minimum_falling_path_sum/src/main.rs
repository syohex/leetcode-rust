fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut dp = vec![vec![None; cols]; rows];
    for i in 0..cols {
        dp[0][i] = Some(matrix[0][i]);
    }

    let steps = [-1, 0, 1];
    for i in 1..rows {
        for j in 0..cols {
            for s in &steps {
                let idx = j as i32 + s;
                if idx >= 0 && (idx as usize) < cols {
                    let idx = idx as usize;
                    let sum = dp[i - 1][idx].unwrap() + matrix[i][j];
                    if let Some(v) = dp[i][j] {
                        dp[i][j] = Some(std::cmp::min(v, sum));
                    } else {
                        dp[i][j] = Some(sum);
                    }
                }
            }
        }
    }

    let mut ret = std::i32::MAX;
    for i in 0..cols {
        if let Some(v) = dp[rows - 1][i] {
            ret = std::cmp::min(ret, v);
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
fn test() {
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
