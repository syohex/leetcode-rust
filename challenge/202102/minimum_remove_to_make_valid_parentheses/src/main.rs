fn min_remove_to_make_valid(s: String) -> String {
    let mut stack: Vec<(usize, char)> = vec![];
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => stack.push((i, c)),
            ')' => {
                if !stack.is_empty() && stack[stack.len() - 1].1 == '(' {
                    stack.pop();
                } else {
                    stack.push((i, c));
                }
            }
            _ => {}
        }
    }

    if stack.is_empty() {
        return s;
    }

    let mut ret = String::new();
    let mut pos = 0;
    for (i, c) in s.chars().enumerate() {
        if pos < stack.len() && i == stack[pos].0 {
            pos += 1;
        } else {
            ret.push(c);
        }
    }

    ret
}

fn main() {
    let ret = min_remove_to_make_valid("lee(t(c)o)de)".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_min_remove_to_make_valid() {
    assert_eq!(
        min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
        "lee(t(c)o)de".to_string()
    );
    assert_eq!(
        min_remove_to_make_valid("a)b(c)d".to_string()),
        "ab(c)d".to_string()
    );
    assert_eq!(min_remove_to_make_valid("))((".to_string()), "".to_string());
    assert_eq!(
        min_remove_to_make_valid("(a(b(c)d)".to_string()),
        "a(b(c)d)".to_string()
    );
}
