fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let cs1: Vec<char> = text1.chars().collect();
    let cs2: Vec<char> = text2.chars().collect();

    let mut dp = vec![vec![0; cs2.len() + 1]; cs1.len() + 1];

    for i in (0..cs1.len()).rev() {
        for j in (0..cs2.len()).rev() {
            if cs1[i] == cs2[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j + 1]);
            }
        }
    }

    dp[0][0]
}

fn main() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    let ret = longest_common_subsequence(text1, text2);
    println!("ret={ret}");
}

#[test]
fn test_longest_common_subsequence() {
    {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        let ret = longest_common_subsequence(text1, text2);
        assert_eq!(ret, 3);
    }
    {
        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        let ret = longest_common_subsequence(text1, text2);
        assert_eq!(ret, 3);
    }
    {
        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        let ret = longest_common_subsequence(text1, text2);
        assert_eq!(ret, 0);
    }
}
