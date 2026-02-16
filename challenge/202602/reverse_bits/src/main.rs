fn reverse_bits(n: i32) -> i32 {
    let mut ret = 0;
    let mut n = n;
    for _ in 0..32 {
        ret = (ret << 1) + (n & 1);
        n >>= 1;
    }

    ret
}

fn main() {
    let ret = reverse_bits(43261596);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(reverse_bits(43261596), 964176192);
    assert_eq!(reverse_bits(2147483644), 1073741822);
}
