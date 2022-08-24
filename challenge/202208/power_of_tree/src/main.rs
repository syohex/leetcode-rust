fn is_power_of_three(n: i32) -> bool {
    let n = n as i64;
    let mut v = 1i64;
    let limit = std::i32::MAX as i64;

    while v <= limit {
        if n == v {
            return true;
        }
        if n < v {
            return false;
        }

        v *= 3;
    }

    false
}

fn main() {
    let ret = is_power_of_three(27);
    println!("ret={ret}");
}

#[test]
fn test_is_power_of_three() {
    assert!(is_power_of_three(27));
    assert!(!is_power_of_three(0));
    assert!(is_power_of_three(9));
    assert!(!is_power_of_three(19682));
    assert!(is_power_of_three(243));
}