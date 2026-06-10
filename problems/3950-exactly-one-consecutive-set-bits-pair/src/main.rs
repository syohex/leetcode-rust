fn consecutive_set_bits(n: i32) -> bool {
    let mut count = 0;
    let mut n = n;
    let mut pairs = 0;
    while n > 0 {
        let m = n % 2;
        if m == 1 {
            count += 1;
        } else {
            if count == 2 {
                pairs += 1;
            } else if count > 2 {
                return false;
            }
            count = 0;
        }

        n /= 2;
    }

    if count == 2 {
        pairs += 1;
    } else if count > 2 {
        return false;
    }

    pairs == 1
}

fn main() {
    let ret = consecutive_set_bits(6);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!consecutive_set_bits(111));
    assert!(consecutive_set_bits(6));
    assert!(consecutive_set_bits(3));
    assert!(consecutive_set_bits(12));
    assert!(!consecutive_set_bits(5));
    assert!(!consecutive_set_bits(7));
    assert!(!consecutive_set_bits(16));
}
