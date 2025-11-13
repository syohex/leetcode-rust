fn max_operations(s: String) -> i32 {
    let cs: Vec<_> = s.chars().collect();

    let mut ret = 0;
    let mut ones = 0;
    let mut i = 0;

    while i < cs.len() {
        if cs[i] == '0' {
            while i + 1 < cs.len() && cs[i + 1] == '0' {
                i += 1;
            }
            ret += ones;
        } else {
            ones += 1;
        }

        i += 1;
    }

    ret
}

fn main() {
    let ret = max_operations("1001101".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_operations("1001101".to_string()), 4);
    assert_eq!(max_operations("000111".to_string()), 0);
}
