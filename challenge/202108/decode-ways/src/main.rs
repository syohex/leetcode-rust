fn num_decoding(s: String) -> i32 {
    let len = s.len();
    let bs: Vec<u8> = s.bytes().collect();
    let mut dp = vec![0; len + 1];
    dp[0] = 1;

    for i in 1..=len {
        if bs[i - 1] > b'0' {
            dp[i] += dp[i - 1];
        }

        if i >= 2 {
            if bs[i - 2] == b'1' {
                dp[i] += dp[i - 2];
            } else if bs[i - 2] == b'2' && bs[i - 1] <= b'6' {
                dp[i] += dp[i - 2];
            }
        }

        if dp[i] == 0 {
            return 0;
        }
    }

    dp[len]
}

fn main() {
    let ret = num_decoding("12".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_num_decodings() {
    assert_eq!(num_decoding("12".to_string()), 2);
    assert_eq!(num_decoding("226".to_string()), 3);
    assert_eq!(num_decoding("0".to_string()), 0);
    assert_eq!(num_decoding("06".to_string()), 0);
}
