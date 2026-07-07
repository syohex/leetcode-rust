fn sum_and_multiply(n: i32) -> i64 {
    let (num, sum) = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .filter(|c| *c != 0)
        .fold((0i64, 0i64), |(num, sum), n| (num * 10 + n, sum + n));
    num * sum
}

fn main() {
    let ret = sum_and_multiply(10203004);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sum_and_multiply(10203004), 12340);
    assert_eq!(sum_and_multiply(1000), 1);
}
