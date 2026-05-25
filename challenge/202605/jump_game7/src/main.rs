fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
    let cs: Vec<_> = s.chars().collect();
    let len = s.len();
    let mut reachables = 0;
    let mut dp = vec![false; len];
    dp[0] = true;

    for i in 1..len {
        let i = i as i32;
        let left = i - max_jump;
        if left - 1 >= 0 && dp[(left - 1) as usize] {
            reachables -= 1;
        }

        let right = i - min_jump;
        if right >= 0 && dp[right as usize] {
            reachables += 1;
        }

        if cs[i as usize] == '0' && reachables > 0 {
            dp[i as usize] = true;
        }
    }

    dp[len - 1]
}

fn main() {
    let ret = can_reach("011010".to_string(), 2, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(can_reach("011010".to_string(), 2, 3));
    assert!(!can_reach("01101110".to_string(), 2, 3));
}
