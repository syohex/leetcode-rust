fn min_partitions(n: String) -> i32 {
    n.bytes()
        .fold(0, |ret, b| std::cmp::max(ret, (b - b'0') as i32))
}

fn main() {
    let ret = min_partitions("32".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_min_partitions() {
    assert_eq!(min_partitions("32".to_string()), 3);
    assert_eq!(min_partitions("82734".to_string()), 8);
    assert_eq!(min_partitions("27346209830709182346".to_string()), 9);
}
