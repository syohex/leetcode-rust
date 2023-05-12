fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let len = questions.len();
    if len == 1 {
        return questions[0][0] as i64;
    }

    let mut dp = vec![0i64; len];

    dp[len - 1] = questions[len - 1][0] as i64;
    for i in (0..=len - 2).rev() {
        dp[i] = questions[i][0] as i64;

        let skip = questions[i][1] as usize;
        if i + skip + 1 < len {
            dp[i] += dp[i + skip + 1];
        }

        dp[i] = std::cmp::max(dp[i], dp[i + 1]);
    }

    dp[0]
}

fn main() {
    let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
    let ret = most_points(questions);
    println!("ret={ret}");
}

#[test]
fn test_most_points() {
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
    {
        let questions = vec![
            vec![12, 46],
            vec![78, 19],
            vec![63, 15],
            vec![79, 62],
            vec![13, 10],
        ];
        let ret = most_points(questions);
        assert_eq!(ret, 79);
    }
}
