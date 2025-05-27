fn difference_of_sums(n: i32, m: i32) -> i32 {
    (1..=n).fold(0, |acc, v| if v % m != 0 { acc + v } else { acc - v })
}

fn main() {
    let ret = difference_of_sums(10, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(difference_of_sums(10, 3), 19);
    assert_eq!(difference_of_sums(5, 6), 15);
    assert_eq!(difference_of_sums(5, 1), -15);
}
