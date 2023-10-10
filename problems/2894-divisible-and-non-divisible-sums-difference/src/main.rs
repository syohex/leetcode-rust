fn difference_of_sums(n: i32, m: i32) -> i32 {
    let num1: i32 = (1..=n).filter(|i| i % m != 0).sum();
    let num2: i32 = (1..=n).filter(|i| i % m == 0).sum();

    num1 - num2
}
fn main() {
    let ret = difference_of_sums(10, 3);
    println!("ret={ret}");
}

#[test]
fn test_difference_of_sums() {
    assert_eq!(difference_of_sums(10, 3), 19);
    assert_eq!(difference_of_sums(5, 6), 15);
    assert_eq!(difference_of_sums(5, 1), -15);
}
