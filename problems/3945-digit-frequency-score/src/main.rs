fn digit_frequency_score(n: i32) -> i32 {
    let mut n = n;
    let mut ret = 0;
    while n > 0 {
        ret += n % 10;
        n /= 10;
    }

    ret
}

fn main() {
    let ret = digit_frequency_score(122);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(digit_frequency_score(122), 5);
    assert_eq!(digit_frequency_score(101), 2);
}
