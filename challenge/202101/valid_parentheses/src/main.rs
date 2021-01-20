fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            '[' | '(' | '{' => stack.push(c),
            ']' => match stack.last() {
                Some(&p) => {
                    if p != '[' {
                        return false;
                    }

                    stack.pop();
                }
                None => {
                    return false;
                }
            },
            ')' => match stack.last() {
                Some(&p) => {
                    if p != '(' {
                        return false;
                    }

                    stack.pop();
                }
                None => {
                    return false;
                }
            },
            '}' => match stack.last() {
                Some(&p) => {
                    if p != '{' {
                        return false;
                    }

                    stack.pop();
                }
                None => {
                    return false;
                }
            },
            _ => (),
        }
    }

    stack.is_empty()
}

fn main() {
    println!("is_valid('()') = {}", is_valid("()".to_string()));
}

#[test]
fn test_is_valid() {
    assert!(is_valid("()".to_string()));
    assert!(is_valid("()[]{}".to_string()));
    assert!(!is_valid("(]".to_string()));
    assert!(!is_valid("([)]".to_string()));
    assert!(is_valid("{[]}".to_string()));
}
