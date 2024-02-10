fn count_substrings(s: String) -> i32 {
    let cs: Vec<char> = s.chars().collect();
    let len = s.len();

    let mut dp = vec![vec![false; len]; len];
    let mut ret = 0;

    for i in 0..len {
        dp[i][i] = true;
        ret += 1;
    }

    for i in 0..(len - 1) {
        if cs[i] == cs[i + 1] {
            dp[i][i + 1] = true;
            ret += 1;
        }
    }

    for i in 3..=len {
        for j in 0..=(len - i) {
            let k = i + j - 1;
            if dp[j + 1][k - 1] && cs[j] == cs[k] {
                dp[j][k] = true;
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let ret = count_substrings("aaa".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_substrings("abc".to_string()), 3);
    assert_eq!(count_substrings("aaa".to_string()), 6);
}
