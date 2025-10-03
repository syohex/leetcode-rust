fn decimal_representation(n: i32) -> Vec<i32> {
    let mut ret = vec![];
    let mut n = n;
    let mut base = 1;

    while n > 0 {
        let m = n % 10;
        if m != 0 {
            ret.push(m * base);
        }
        n /= 10;
        base *= 10;
    }

    ret.into_iter().rev().collect()
}

fn main() {
    let ret = decimal_representation(10234567);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(decimal_representation(537), vec![500, 30, 7]);
    assert_eq!(decimal_representation(102), vec![100, 2]);
    assert_eq!(decimal_representation(6), vec![6]);
}
