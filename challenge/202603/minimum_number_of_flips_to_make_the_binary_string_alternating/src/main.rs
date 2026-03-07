fn min_flips(s: String) -> i32 {
    fn f(s: &str) -> i32 {
        let mut zero_start = 0;
        let mut one_start = 0;

        let mut n0 = '0';
        let mut n1 = '1';
        for c in s.chars() {
            if c != n0 {
                zero_start += 1;
            }
            if c != n1 {
                one_start += 1;
            }

            n0 = if n0 == '0' { '1' } else { '0' };
            n1 = if n1 == '0' { '1' } else { '0' };
        }

        std::cmp::min(zero_start, one_start)
    }

    let len = s.len();
    let s = format!("{s}{s}");
    let mut i = 0;
    let mut ret = i32::MAX;

    while i < len {
        ret = std::cmp::min(ret, f(&s[i..(i + len)]));
        i += 1;
    }

    ret
}

fn main() {
    let ret = min_flips("1111111111111111111111111".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_flips("01001001101".to_string()), 2);
    assert_eq!(min_flips("111000".to_string()), 2);
    assert_eq!(min_flips("010".to_string()), 0);
    assert_eq!(min_flips("1110".to_string()), 1);
}
