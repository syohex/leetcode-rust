fn min_bit_flips(start: i32, goal: i32) -> i32 {
    (start ^ goal).count_ones() as i32
}

fn main() {
    let ret = min_bit_flips(3, 4);
    println!("ret={ret}");
}

#[test]
fn test_min_bit_flips() {
    assert_eq!(min_bit_flips(10, 7), 3);
    assert_eq!(min_bit_flips(3, 4), 3);
}
