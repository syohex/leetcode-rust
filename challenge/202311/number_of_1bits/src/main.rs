fn hamming_weight(n: u32) -> i32 {
    let n = (n & 0x55555555) + ((n >> 1) & 0x55555555);
    let n = (n & 0x33333333) + ((n >> 2) & 0x33333333);
    let n = (n & 0x0F0F0F0F) + ((n >> 4) & 0x0F0F0F0F);
    let n = (n & 0x00FF00FF) + ((n >> 8) & 0x00FF00FF);
    let n = (n & 0x0000FFFF) + ((n >> 16) & 0x0000FFFF);
    n as i32
}

fn main() {
    let ret = hamming_weight(0xDEADBEEF);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(hamming_weight(0b1011), 3);
    assert_eq!(hamming_weight(4096), 1);
    assert_eq!(hamming_weight(0xFFFFFFFD), 31);
}
