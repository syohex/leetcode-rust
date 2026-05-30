fn valid_digit(n: i32, x: i32) -> bool {
    let ns: Vec<_> = n.to_string().chars().collect();
    let cx = char::from_digit(x as u32, 10).unwrap();
    ns[0] != cx && ns.into_iter().skip(1).filter(|m| *m == cx).count() >= 1
}

fn main() {
    let ret = valid_digit(101, 0);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(valid_digit(344, 4));
    assert!(!valid_digit(10, 2));
    assert!(valid_digit(101, 0));
    assert!(!valid_digit(232, 2));
    assert!(!valid_digit(5, 1));
}
