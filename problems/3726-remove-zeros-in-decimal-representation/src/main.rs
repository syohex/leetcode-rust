fn remove_zeros(n: i64) -> i64 {
    n.to_string()
        .bytes()
        .filter(|&c| c != b'0')
        .fold(0, |acc, b| acc * 10 + (b - b'0') as i64)
}

fn main() {
    let ret = remove_zeros(10203040506070809i64);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(remove_zeros(1020030), 123);
    assert_eq!(remove_zeros(1), 1);
}
