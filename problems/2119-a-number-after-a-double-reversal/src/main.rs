fn is_same_after_reversals(num: i32) -> bool {
    let mut num = num;
    let mut v = Vec::new();
    while num > 0 {
        v.push(num % 10);
        num /= 10;
    }

    if v.len() <= 1 {
        true
    } else {
        v[0] != 0
    }
}

fn main() {
    println!("ret={}", is_same_after_reversals(526));
    println!("ret={}", is_same_after_reversals(1800));
    println!("ret={}", is_same_after_reversals(0));
}

#[test]
fn test_is_same_after_reversals() {
    assert!(is_same_after_reversals(526));
    assert!(!is_same_after_reversals(1800));
    assert!(is_same_after_reversals(0));
    assert!(is_same_after_reversals(9));
}
