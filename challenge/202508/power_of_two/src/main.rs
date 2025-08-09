fn is_power_of_two(n: i32) -> bool {
    n >= 1 && (n & (n - 1)) == 0
}

fn main() {
    let ret = is_power_of_two(256);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_power_of_two(1));
    assert!(is_power_of_two(16));
    assert!(!is_power_of_two(3));
    assert!(!is_power_of_two(-2));
    assert!(!is_power_of_two(0));
}
