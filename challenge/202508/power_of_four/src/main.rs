fn is_power_of_four(n: i32) -> bool {
    n >= 1 && ((n & (n - 1)) == 0) && ((n & 0x55555555) != 0)
}

fn main() {
    let ret = is_power_of_four(256);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_power_of_four(16));
    assert!(!is_power_of_four(32));
    assert!(!is_power_of_four(5));
    assert!(is_power_of_four(1));
}
