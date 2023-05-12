fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let len = questions.len();
    let mut dp = vec![0i64; len];
    let mut ret = 0i64;

    for i in 0..len {
        let point = questions[i][0] as i64;
        dp[i] = point;
        for j in 0..i {
            if questions[j][1] as usize + j < i {
                dp[i] = std::cmp::max(dp[i], dp[j] + point);
            }
        }

        ret = std::cmp::max(ret, dp[i]);
    }

    ret
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
