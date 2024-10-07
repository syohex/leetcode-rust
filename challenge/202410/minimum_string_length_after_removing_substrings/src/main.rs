fn min_length(s: String) -> i32 {
    let mut v: Vec<char> = vec![];

    for c in s.chars() {
        if c == 'B' {
            if !v.is_empty() && *v.last().unwrap() == 'A' {
                v.pop();
                continue;
            }
        } else if c == 'D' {
            if !v.is_empty() && *v.last().unwrap() == 'C' {
                v.pop();
                continue;
            }
        }

        v.push(c);
    }

    v.len() as i32
}

fn main() {
    let ret = min_length("ABFCACDB".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_length("ABFCACDB".to_string()), 2);
    assert_eq!(min_length("ACBBD".to_string()), 5);
}
