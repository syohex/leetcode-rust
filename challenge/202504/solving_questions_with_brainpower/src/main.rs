fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let len = questions.len();
    let mut dp = vec![0i64; len];
    dp[len - 1] = questions[len-1][0] as i64;

    for i in (0..(len - 1)).rev() {
        let mut v = questions[i][0] as i64;
        let skips = questions[i][1] as usize + 1;
        if i + skips < len {
            v += dp[i + skips];
        }

        dp[i] = std::cmp::max(v, dp[i + 1]);
    }

    dp[0]
}

fn main() {
    let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
    let ret = most_points(questions);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let questions = vec![vec![3, 2]];
        let ret = most_points(questions);
        assert_eq!(ret, 3);
    }
    {
        let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        let ret = most_points(questions);
        assert_eq!(ret, 5);
    }
    {
        let questions = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        let ret = most_points(questions);
        assert_eq!(ret, 7);
    }
}
