fn total_money(n: i32) -> i32 {
    let mut base = 1;
    let mut n = n;
    let mut ret = 0;

    while n > 0 {
        let m = std::cmp::min(7, n);
        ret += ((base + base + m - 1) * m) / 2;

        n -= m;
        base += 1;
    }

    ret
}

fn main() {
    let ret = total_money(20);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(total_money(4), 10);
    assert_eq!(total_money(10), 37);
    assert_eq!(total_money(20), 96);
}
