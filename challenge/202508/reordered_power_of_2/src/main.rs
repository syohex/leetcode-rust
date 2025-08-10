fn reordered_power_of2(n: i32) -> bool {
    use std::collections::HashSet;

    fn f(n: u32) -> Vec<char> {
        let s = n.to_string();
        let mut v : Vec<_> = s.chars().collect();
        v.sort_unstable();
        v
    }

    let primes : HashSet<_> = (0..=31).map(|n| f(2u32.pow(n))).collect();
    primes.contains(&f(n as u32))
}

fn main() {
    let ret = reordered_power_of2(46);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(reordered_power_of2(1));
    assert!(!reordered_power_of2(10));
    assert!(reordered_power_of2(56635));
}
