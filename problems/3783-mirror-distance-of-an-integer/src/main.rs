fn mirror_distance(n: i32) -> i32 {
    let rev = n.to_string().chars().rev().fold(0, |acc, c| {
        acc * 10 + c.to_digit(10).unwrap()
    }) as i32;

    (n - rev).abs()
}

fn main() {
    let ret = mirror_distance(25);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(mirror_distance(25), 27);
    assert_eq!(mirror_distance(10), 9);
    assert_eq!(mirror_distance(7), 0);
}
