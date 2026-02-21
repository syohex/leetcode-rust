fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    use std::collections::HashSet;

    let primes: HashSet<_> = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]
        .into_iter()
        .collect();
    (left..=right)
        .filter(|&n| primes.contains(&n.count_ones()))
        .count() as i32
}

fn main() {
    let ret = count_prime_set_bits(10, 15);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_prime_set_bits(6, 10), 4);
    assert_eq!(count_prime_set_bits(10, 15), 5);
}
