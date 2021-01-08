fn is_valid(s: String) -> bool {
    let mut v: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            ')' => {
                if let Some(p) = v.pop() {
                    if p != '(' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            '}' => {
                if let Some(p) = v.pop() {
                    if p != '{' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ']' => {
                if let Some(p) = v.pop() {
                    if p != '[' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {
                v.push(c);
            }
        }
    }

    v.is_empty()
}

fn main() {
    println!("is_valid(\"\") = {}", is_valid("\"\"".to_string()));
}

#[test]
fn test_is_valid() {
    assert!(is_valid("()".to_string()));
    assert!(is_valid("()[]{}".to_string()));
    assert!(!is_valid("(]".to_string()));
    assert!(!is_valid("([)]".to_string()));
    assert!(is_valid("{[]}".to_string()));
}
