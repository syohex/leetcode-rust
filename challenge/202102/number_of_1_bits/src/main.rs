fn hamming_weight(n: u32) -> i32 {
    let n = (n & 0x55555555) + ((n >> 1) & 0x55555555);
    let n = (n & 0x33333333) + ((n >> 2) & 0x33333333);
    let n = (n & 0x0f0f0f0f) + ((n >> 4) & 0x0f0f0f0f);
    let n = (n & 0x00ff00ff) + ((n >> 8) & 0x00ff00ff);
    ((n & 0x0000ffff) + ((n >> 16) & 0x0000ffff)) as i32
}

fn main() {
    println!("hamming_weight(0xff)={}", hamming_weight(0xff));
}

#[test]
fn test_hamming_weight() {
    assert_eq!(hamming_weight(0b00000000000000000000000000001011), 3);
    assert_eq!(hamming_weight(0b00000000000001000000000000000000), 1);
    assert_eq!(hamming_weight(0b11111111111111111111111111111101), 31);
}
