fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    let (rows, cols) = (matrix.len(), matrix[0].len());
    let mut dp = vec![vec![0; cols]; rows];

    let mut ret = 0;
    for i in 0..rows {
        for j in 0..cols {
            if matrix[i][j] == '0' {
                continue;
            }

            if j == 0 {
                dp[i][j] = 1;
            } else {
                dp[i][j] = dp[i][j - 1] + 1;
            }

            let mut min_width = dp[i][j];
            for k in (0..=i).rev() {
                if matrix[k][j] == '0' {
                    break;
                }

                min_width = std::cmp::min(min_width, dp[k][j]);
                ret = std::cmp::max(ret, min_width * (i - k + 1) as i32);
            }
        }
    }

    ret
}

fn main() {
    let matrix = vec![
        vec!['1', '0', '1', '0', '0'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '0', '1', '0'],
    ];
    let ret = maximal_rectangle(matrix);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        let ret = maximal_rectangle(matrix);
        assert_eq!(ret, 6);
    }
    {
        let matrix = vec![vec!['0']];
        let ret = maximal_rectangle(matrix);
        assert_eq!(ret, 0);
    }
    {
        let matrix = vec![vec!['1']];
        let ret = maximal_rectangle(matrix);
        assert_eq!(ret, 1);
    }
}
