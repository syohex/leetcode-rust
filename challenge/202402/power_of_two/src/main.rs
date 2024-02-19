fn is_power_of_two(n: i32) -> bool {
    n >= 1 && n.count_ones() == 1
}

fn main() {
    println!("ret={}", is_power_of_two(256));
}

#[test]
fn test() {
    assert!(is_power_of_two(1));
    assert!(is_power_of_two(16));
    assert!(!is_power_of_two(3));
}
