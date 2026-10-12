fn min_rotations(s: String) -> i32 {
    s.bytes()
        .map(|b| (b - b'0') as i32)
        .fold((0, 0), |(acc, pos), n| {
            let diff = (n - pos).abs();
            let diff = if diff > 5 { 10 - diff } else { diff };
            (acc + diff, n)
        })
        .0
}
fn main() {
    let ret = min_rotations("0192837465".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_rotations("0192837465".to_string()), 25);
    assert_eq!(min_rotations("1200210200".to_string()), 12);
}
