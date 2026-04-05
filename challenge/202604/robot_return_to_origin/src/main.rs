fn judge_circle(moves: String) -> bool {
    moves.chars().fold((0, 0), |(x, y), c| match c {
        'L' => (x - 1, y),
        'R' => (x + 1, y),
        'U' => (x, y + 1),
        'D' => (x, y - 1),
        _ => unreachable!("never reach here"),
    }) == (0, 0)
}

fn main() {
    let ret = judge_circle("UD".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(judge_circle("UD".to_string()));
    assert!(!judge_circle("LL".to_string()));
}
