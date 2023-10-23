fn is_power_of_four(n: i32) -> bool {
    let bits = n.count_ones();
    bits == 1 && (n as u32 & 0xAAAAAAAA) == 0
}

fn main() {
    let ret = is_power_of_four(64);
    println!("ret={ret}");
}

#[test]
fn test_is_power_of_four() {
    assert!(is_power_of_four(1));
    assert!(is_power_of_four(16));
    assert!(!is_power_of_four(5));
    assert!(!is_power_of_four(6));
    assert!(!is_power_of_four(19));
    assert!(!is_power_of_four(0));
}
