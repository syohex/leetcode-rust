fn bitwise_complement(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    let mut n = n;
    let mut ret = 0;
    let mut power = 1;

    while n > 0 {
        let v = if (n & 1) == 1 { 0 } else { 1 };
        ret += v * power;

        n >>= 1;
        power <<= 1;
    }

    ret
}

fn main() {
    let ret = bitwise_complement(0xffff);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(bitwise_complement(0), 1);
    assert_eq!(bitwise_complement(5), 2);
    assert_eq!(bitwise_complement(7), 0);
    assert_eq!(bitwise_complement(10), 5);
}
