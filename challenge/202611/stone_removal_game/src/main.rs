fn can_alice_win(n: i32) -> bool {
    let mut stones = 10;
    let mut n = n;
    let mut is_alice = true;

    loop {
        if n < stones {
            return !is_alice;
        }

        n -= stones;
        stones -= 1;
        is_alice = !is_alice;
    }
}
fn main() {
    let ret = can_alice_win(12);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(can_alice_win(12));
    assert!(!can_alice_win(1));
}
