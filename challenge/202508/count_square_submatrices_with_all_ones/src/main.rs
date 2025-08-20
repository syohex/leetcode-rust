fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    use std::cmp::min;

    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut dp = vec![vec![0; cols + 1]; rows + 1];
    let mut ret = 0;

    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == 1 {
                dp[i + 1][j + 1] = 1 + min(dp[i][j], min(dp[i + 1][j], dp[i][j + 1]));
                ret += dp[i + 1][j + 1];
            }
        }
    }

    ret
}

fn main() {
    let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
    let ret = count_squares(matrix);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let matrix = vec![vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![0, 1, 1, 1]];
        let ret = count_squares(matrix);
        assert_eq!(ret, 15);
    }
    {
        let matrix = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
        let ret = count_squares(matrix);
        assert_eq!(ret, 7);
    }
}
