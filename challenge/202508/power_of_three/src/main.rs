fn is_power_of_three(n: i32) -> bool {
    // 3^19 is the biggest number of 3^x within 2^32
    n >= 1 && 3i32.pow(19) % n == 0
}

fn main() {
    let ret = is_power_of_three(81);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_power_of_three(27));
    assert!(!is_power_of_three(0));
    assert!(!is_power_of_three(-1));
}
