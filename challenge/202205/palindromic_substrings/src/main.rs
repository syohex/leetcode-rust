fn count_substrings(s: String) -> i32 {
    let len = s.len();
    let bs: Vec<u8> = s.bytes().collect();

    let mut dp = vec![vec![false; len]; len];
    let mut ret = 0;

    for i in 0..len {
        dp[i][i] = true;
        ret += 1;
    }

    for i in 0..len - 1 {
        dp[i][i + 1] = bs[i] == bs[i + 1];
        if dp[i][i + 1] {
            ret += 1;
        }
    }

    for i in 3..=len {
        for j in 0..=(len - i) {
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
    let s = "abc".to_string();
    let ret = count_substrings(s);
    println!("ret={ret}");
}

#[test]
fn test_counst_substrings() {
    {
        let s = "abc".to_string();
        assert_eq!(count_substrings(s), 3);
    }
    {
        let s = "aaa".to_string();
        assert_eq!(count_substrings(s), 6);
    }
}
