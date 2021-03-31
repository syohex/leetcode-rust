fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    use std::cmp::max;

    let mut envelopes = envelopes;
    envelopes.sort_by(|a, b| {
        if a[0] == b[0] {
            a[1].cmp(&b[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

    let mut dp = vec![1; envelopes.len()];
    for i in 0..envelopes.len() {
        for j in 0..i {
            if envelopes[j][0] < envelopes[i][0] && envelopes[j][1] < envelopes[i][1] {
                dp[i] = max(dp[i], dp[j] + 1);
            }
        }
    }

    *dp.iter().max().unwrap()
}

fn main() {
    let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
    let ret = max_envelopes(envelopes);
    println!("ret={}", ret);
}

#[test]
fn test_max_envelopes() {
    {
        let envelopes = vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]];
        assert_eq!(max_envelopes(envelopes), 3);
    }
    {
        let envelopes = vec![vec![1, 1], vec![1, 1], vec![1, 1]];
        assert_eq!(max_envelopes(envelopes), 1);
    }
}
