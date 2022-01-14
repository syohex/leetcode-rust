fn my_atoi(s: String) -> i32 {
    let bs = s.bytes().skip_while(|b| *b == b' ').collect::<Vec<u8>>();
    if bs.is_empty() {
        return 0;
    }

    let mut pos = 0;
    let negative = if bs[pos] == b'+' {
        pos += 1;
        false
    } else if bs[pos] == b'-' {
        pos += 1;
        true
    } else {
        false
    };

    let mut ret = 0i64;
    let int_min = std::i32::MIN as i64;
    let int_max = std::i32::MAX as i64;
    let limit = bs.len();

    while pos < limit && bs[pos] >= b'0' && bs[pos] <= b'9' {
        ret = 10 * ret + (bs[pos] - b'0') as i64;
        pos += 1;

        if negative {
            if -ret <= int_min {
                return int_min as i32;
            }
        } else {
            if ret >= int_max {
                return int_max as i32;
            }
        }
    }

    if negative {
        -ret as i32
    } else {
        ret as i32
    }
}

fn main() {
    let ret = my_atoi("-9147483648".to_string());
    println!("ret={ret}");
}

#[test]
fn test_my_atoi() {
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi("    -42".to_string()), -42);
    assert_eq!(my_atoi("4913 with words".to_string()), 4913);
    assert_eq!(my_atoi(" -9147483648".to_string()), -2147483648);
}
