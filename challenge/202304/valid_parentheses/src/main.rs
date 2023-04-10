fn is_valid(s: String) -> bool {
    let mut v = vec![];
    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            v.push(c);
        } else if c == ')' {
            if let Some(d) = v.pop() {
                if d != '(' {
                    return false;
                }
            } else {
                return false;
            }
        } else if c == ']' {
            if let Some(d) = v.pop() {
                if d != '[' {
                    return false;
                }
            } else {
                return false;
            }
        } else if c == '}' {
            if let Some(d) = v.pop() {
                if d != '{' {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    v.is_empty()
}

fn main() {
    println!("ret={}", is_valid("(())()".to_string()));
}

#[test]
fn test_is_valid() {
    assert!(is_valid("()".to_string()));
    assert!(is_valid("()[]{}".to_string()));
    assert!(!is_valid("(]".to_string()));
    assert!(!is_valid("]".to_string()));
}
