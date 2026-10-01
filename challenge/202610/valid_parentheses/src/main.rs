fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => {
                stack.push(c);
            }
            ')' => {
                if let Some(d) = stack.last()
                    && *d == '('
                {
                    stack.pop();
                } else {
                    return false;
                }
            }
            '}' => {
                if let Some(d) = stack.last()
                    && *d == '{'
                {
                    stack.pop();
                } else {
                    return false;
                }
            }
            ']' => {
                if let Some(d) = stack.last()
                    && *d == '['
                {
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => unreachable!("never reach here"),
        }
    }

    stack.is_empty()
}

fn main() {
    let ret = is_valid("((((((()))))))".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(!is_valid("]".to_string()));
    assert!(is_valid("()".to_string()));
    assert!(is_valid("()[]{}".to_string()));
    assert!(!is_valid("(]".to_string()));
    assert!(is_valid("([])".to_string()));
    assert!(!is_valid("([)]".to_string()));
}
