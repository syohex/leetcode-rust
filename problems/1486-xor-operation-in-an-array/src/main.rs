fn xor_operation(n: i32, start: i32) -> i32 {
    (1..n)
        .into_iter()
        .map(|m| start + (2 * m))
        .fold(start, |acc, x| acc ^ x)
}

fn main() {
    println!("xor_operation(5, 0) = {}", xor_operation(5, 0));
}

#[test]
fn test_xor_operation() {
    assert_eq!(xor_operation(5, 0), 8);
    assert_eq!(xor_operation(4, 3), 8);
    assert_eq!(xor_operation(1, 7), 7);
    assert_eq!(xor_operation(10, 5), 2);
}
