fn minimum_steps(s: String) -> i64 {
    let mut blacks = 0;
    let mut ret = 0;

    for c in s.chars() {
        if c == '0' {
            ret += blacks;
        } else {
            blacks += 1;
        }
    }

    ret
}

fn main() {
    let ret = minimum_steps("1010".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_steps("101".to_string()), 1);
    assert_eq!(minimum_steps("100".to_string()), 2);
    assert_eq!(minimum_steps("0111".to_string()), 0);
}
