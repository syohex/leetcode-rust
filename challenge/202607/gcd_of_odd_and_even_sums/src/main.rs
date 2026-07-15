fn gcd_of_odd_even_sums(n: i32) -> i32 {
    n
}

fn main() {
    let ret = gcd_of_odd_even_sums(5);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(gcd_of_odd_even_sums(4), 4);
    assert_eq!(gcd_of_odd_even_sums(5), 5);
}
