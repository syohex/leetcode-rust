fn has_alternating_bits(n: i32) -> bool {
    let mut prev = n & 1;
    let mut n = n >> 1;

    while n > 0 {
        let b = n & 1;
        if b == prev {
            return false;
        }

        prev = b;
        n >>= 1;
    }

    true
}

fn main() {
    println!("ret={}", has_alternating_bits(10));
}

#[test]
fn test() {
    let test_cases = [(5, true), (7, false), (11, false)];
    for (v, expected) in test_cases {
        assert_eq!(has_alternating_bits(v), expected);
    }
}
