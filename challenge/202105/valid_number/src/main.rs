fn is_number(s: String) -> bool {
    let bs: Vec<u8> = s.bytes().collect();
    let mut has_num = false;
    let mut has_dot = false;
    let mut has_e = false;
    for i in 0..bs.len() {
        if bs[i] >= b'0' && bs[i] <= b'9' {
            has_num = true;
        } else if bs[i] == b'+' || bs[i] == b'-' {
            if i >= 1 && !(bs[i - 1] == b'e' || bs[i - 1] == b'E') {
                return false;
            }
        } else if bs[i] == b'e' || bs[i] == b'E' {
            if has_e || !has_num {
                return false;
            }

            has_e = true;
            has_num = false;
        } else if bs[i] == b'.' {
            if has_dot || has_e {
                return false;
            }

            has_dot = true;
        } else {
            return false;
        }
    }

    has_num
}

fn main() {
    println!("ret={}", is_number("-123.456e789".to_string()));
}

#[test]
fn test_is_number() {
    let valids = vec![
        "2",
        "0089",
        "-0.1",
        "+3.14",
        "4.",
        "-.9",
        "2e10",
        "-90E3",
        "3e+7",
        "+6e-1",
        "53.5e93",
        "-123.456e789",
    ];
    for valid in valids {
        assert!(is_number(valid.to_string()));
    }

    let invalids = vec!["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"];
    for invalid in invalids {
        assert!(!is_number(invalid.to_string()));
    }
}
