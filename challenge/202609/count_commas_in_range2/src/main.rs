fn count_commas(n: i64) -> i64 {
    let mut ret = 0i64;
    let mut m = 1_000i64;

    while m <= n {
        ret += n - m + 1;
        m *= 1000;
    }

    ret
}

fn main() {
    let ret = count_commas(1_000_001);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_commas(1_000_000), 999002);
    assert_eq!(count_commas(1002), 3);
    assert_eq!(count_commas(998), 0);
}
