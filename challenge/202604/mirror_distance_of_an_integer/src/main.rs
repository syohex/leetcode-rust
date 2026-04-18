fn mirror_distance(n: i32) -> i32 {
    let mirror = n
        .to_string()
        .chars()
        .rev()
        .skip_while(|c| *c == '0')
        .fold(0i32, |acc, c| acc * 10 + c.to_digit(10).unwrap() as i32);

    (n - mirror).abs()
}

fn main() {
    let ret = mirror_distance(10);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(mirror_distance(25), 27);
    assert_eq!(mirror_distance(10), 9);
    assert_eq!(mirror_distance(7), 0);
}
