fn max_product(n: i32) -> i32 {
    let mut v: Vec<_> = n.to_string().bytes().collect();
    v.sort_unstable();

    v.into_iter()
        .rev()
        .take(2)
        .fold(1, |acc, b| acc * (b - b'0') as i32)
}

fn main() {
    let ret = max_product(798);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_product(31), 3);
    assert_eq!(max_product(22), 4);
    assert_eq!(max_product(124), 8);
}
