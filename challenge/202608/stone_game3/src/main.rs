fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let len = stone_value.len();
    let mut dp = vec![i32::MIN; len + 1];
    dp[len] = 0;

    for i in (0..len).rev() {
        dp[i] = stone_value[i] - dp[i + 1];
        if i + 1 < len {
            dp[i] = std::cmp::max(dp[i], stone_value[i] + stone_value[i + 1] - dp[i + 2]);
        }
        if i + 2 < len {
            dp[i] = std::cmp::max(
                dp[i],
                stone_value[i] + stone_value[i + 1] + stone_value[i + 2] - dp[i + 3],
            );
        }
    }

    match dp[0].cmp(&0) {
        std::cmp::Ordering::Less => "Bob".to_string(),
        std::cmp::Ordering::Equal => "Tie".to_string(),
        std::cmp::Ordering::Greater => "Alice".to_string(),
    }
}

fn main() {
    let ret = stone_game_iii(vec![1, 2, 3, 7]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(stone_game_iii(vec![1, 2, 3, 7]), "Bob");
    assert_eq!(stone_game_iii(vec![1, 2, 3, -9]), "Alice");
    assert_eq!(stone_game_iii(vec![1, 2, 3, 6]), "Tie");
}
