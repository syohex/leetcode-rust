fn count_substrings(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let bs: Vec<u8> = s.bytes().collect();
    let mut dp = vec![vec![false; s.len()]; s.len()];
    let mut ret = 0;
    for i in 0..bs.len() {
        dp[i][i] = true;
        ret += 1;
    }

    for i in 0..bs.len() - 1 {
        dp[i][i + 1] = bs[i] == bs[i + 1];
        if dp[i][i+1] {
            ret += 1;
        }
    }

    for i in 3..=bs.len() {
        let limit_j = bs.len() - i;
        for j in 0..=limit_j {
            let k = j + i - 1;
            dp[j][k] = dp[j + 1][k - 1] && bs[j] == bs[k];
            if dp[j][k] {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let ret = count_substrings("abc".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_count_substrings() {
    assert_eq!(count_substrings("abc".to_string()), 3);
    assert_eq!(count_substrings("aaa".to_string()), 6);
}
