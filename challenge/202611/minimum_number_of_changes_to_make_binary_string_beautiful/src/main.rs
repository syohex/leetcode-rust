fn min_changes(s: String) -> i32 {
    let mut count = 1;
    let cs: Vec<_> = s.chars().collect();
    let len = s.len();

    let mut ret = 0;
    for i in 1..len {
        if cs[i] == cs[i - 1] {
            count += 1;
            continue;
        }

        if count % 2 == 0 {
            count = 1;
        } else {
            count = 2;
            ret += 1;
        }
    }

    ret
}

fn main() {
    let ret = min_changes("1001".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_changes("1001".to_string()), 2);
    assert_eq!(min_changes("10".to_string()), 1);
    assert_eq!(min_changes("0000".to_string()), 0);
}
