fn is_three(n: i32) -> bool {
    use std::collections::HashSet;

    let mut s: HashSet<i32> = HashSet::new();
    let limit = (n as f64).sqrt() as i32;
    for i in 1..=limit {
        if n % i == 0 {
            s.insert(i);
            s.insert(n / i);
        }
    }

    s.len() == 3
}

fn main() {
    println!("ret(2)={}", is_three(2));
    println!("ret(4)={}", is_three(4));
}

#[test]
fn test_is_three() {
    assert!(!is_three(2));
    assert!(is_three(4));
    assert!(is_three(3));
}
