fn hamming_weight(n: u32) -> i32 {
    let mut n = n;
    let mut ret = 0;
    while n > 0 {
        if n & 1 == 1 {
            ret += 1;
        }

        n >>= 1;
    }

    ret
}

fn main() {
    let ret = hamming_weight(0xFE);
    println!("0xFE has {ret} of 1 bits");
}

#[test]
fn test_hamming_weight() {
    assert_eq!(hamming_weight(11), 3);
    assert_eq!(hamming_weight(32), 1);
    assert_eq!(hamming_weight(0xFFFFFFF7), 31);
}
