fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            _ => {
                if let Some(d) = stack.pop() {
                    if !((c == ')' && d == '(') || (c == '}' && d == '{') || (c == ']' && d == '['))
                    {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }

    stack.is_empty()
}

fn main() {
    let ret = is_valid("()[]{}".to_string());
    println!("ret={ret}");
}

#[test]
fn test_is_valid() {
    assert!(is_valid("()".to_string()));
    assert!(is_valid("()[]{}".to_string()));
    assert!(!is_valid("(]".to_string()));
    assert!(!is_valid("]".to_string()));
    assert!(!is_valid("(((((".to_string()));
}
