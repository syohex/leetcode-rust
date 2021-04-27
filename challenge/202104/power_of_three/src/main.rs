fn is_power_of_three(n: i32) -> bool {
    let mut n = n;
    while n != 0 && n % 3 == 0 {
        n /= 3;
    }

    n == 1
}

fn main() {
    println!("is_power_of_three(27)={}", is_power_of_three(27));
}

#[test]
fn test_is_power_of_three() {
    assert!(is_power_of_three(27));
    assert!(!is_power_of_three(0));
    assert!(is_power_of_three(9));
    assert!(!is_power_of_three(45));
}
