fn is_adjacent_diff_at_most_two(s: String) -> bool {
    let bs: Vec<_> = s.bytes().collect();
    for i in 1..bs.len() {
        if (bs[i] as i32 - bs[i - 1] as i32).abs() > 2 {
            return false;
        }
    }
    true
}

fn main() {
    let ret = is_adjacent_diff_at_most_two("123".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_adjacent_diff_at_most_two("132".to_string()));
    assert!(!is_adjacent_diff_at_most_two("129".to_string()));
}
