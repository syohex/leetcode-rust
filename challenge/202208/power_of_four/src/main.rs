fn is_power_of_four(n: i32) -> bool {
    (0u32..=15)
        .into_iter()
        .find(|&i| 4_i32.pow(i) == n)
        .is_some()
}

fn main() {
    let rets: Vec<bool> = [16, 5, 3]
        .into_iter()
        .map(|n| is_power_of_four(n))
        .collect();
    println!("rets={:?}", rets);
}

#[test]
fn test_is_power_of_four() {
    assert!(is_power_of_four(16));
    assert!(is_power_of_four(64));
    assert!(is_power_of_four(1));
    assert!(!is_power_of_four(5));
    assert!(!is_power_of_four(3));
}
