fn count_odds(low: i32, high: i32) -> i32 {
    let half = (high - low) / 2;
    match (low % 2 == 0, high % 2 == 0) {
        (true, true) => half,
        (true, false) | (false, true) => half + 1,
        (false, false) => half + 1,
    }
}

fn main() {
    let ret = count_odds(3, 7);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_odds(0, 10), 5);
    assert_eq!(count_odds(8, 9), 1);
    assert_eq!(count_odds(3, 8), 3);
    assert_eq!(count_odds(8, 10), 1);
    assert_eq!(count_odds(3, 7), 3);
    assert_eq!(count_odds(1, 11), 6);
}
