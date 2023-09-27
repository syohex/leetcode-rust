fn decode_at_index(s: String, k: i32) -> String {
    let mut total_len = 0;

    for c in s.chars() {
        if c.is_numeric() {
            total_len *= c as usize - '0' as usize;
        } else {
            total_len += 1;
        }
    }

    let mut k = k as usize;
    for c in s.chars().rev() {
        k %= total_len;

        if k == 0 && !c.is_numeric() {
            return c.to_string();
        }

        if c.is_numeric() {
            total_len /= c as usize - '0' as usize;
        } else {
            total_len -= 1;
        }
    }

    panic!("never reach here");
}

fn main() {
    let s = "leet2code3".to_string();
    let k = 10;
    let ret = decode_at_index(s, k);
    println!("ret={ret}");
}

#[test]
fn test_decode_at_index() {
    {
        let s = "leet2code3".to_string();
        let k = 10;
        let ret = decode_at_index(s, k);
        assert_eq!(ret, "o");
    }
    {
        let s = "ha22".to_string();
        let k = 5;
        let ret = decode_at_index(s, k);
        assert_eq!(ret, "h");
    }
    {
        let s = "a2345678999999999999999".to_string();
        let k = 1;
        let ret = decode_at_index(s, k);
        assert_eq!(ret, "a");
    }
    {
        let s = "a23".to_string();
        let k = 6;
        let ret = decode_at_index(s, k);
        assert_eq!(ret, "a");
    }
}
