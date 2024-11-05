fn min_changes(s: String) -> i32 {
    let cs: Vec<char> = s.chars().collect();
    let mut prev = cs[0];
    let mut count = 1;
    let mut changes = 0;

    for c in cs.into_iter().skip(1) {
        if c == prev {
            count += 1;
            continue;
        }

        if count % 2 == 0 {
            count = 1;
        } else {
            count = 2;
            changes += 1;
        }

        prev = c;
    }

    changes
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
