fn flower_game(n: i32, m: i32) -> i64 {
    (n as i64 * m as i64) / 2i64
}

fn main() {
    let ret = flower_game(5000, 2032);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(flower_game(3, 2), 3i64);
    assert_eq!(flower_game(1, 1), 0i64);
}
