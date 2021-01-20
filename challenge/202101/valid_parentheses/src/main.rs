macro_rules! check_pair {
    ($s:ident, $pair:literal) => {
        match $s.last() {
            Some(&p) => {
                if p != $pair {
                    return false;
                }

                $s.pop();
            }
            None => {
                return false;
            }
        }
    };
}

fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            '[' | '(' | '{' => stack.push(c),
            ']' => check_pair!(stack, '['),
            ')' => check_pair!(stack, '('),
            '}' => check_pair!(stack, '{'),
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
    assert!(!is_valid("}".to_string()));
}
