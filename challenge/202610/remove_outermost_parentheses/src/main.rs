fn remove_outer_parentheses(s: String) -> String {
    let mut ret = String::new();
    let mut opens = 0;

    for c in s.chars() {
        let is_open = c == '(';
        if !is_open {
            opens -= 1;
        }

        if opens >= 1 {
            ret.push(c);
        }

        if is_open {
            opens += 1;
        }
    }

    ret
}

fn main() {
    let ret = remove_outer_parentheses("(()())(())".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(remove_outer_parentheses("(()())(())".to_string()), "()()()");
    assert_eq!(
        remove_outer_parentheses("(()())(())(()(()))".to_string()),
        "()()()()(())"
    );
    assert_eq!(remove_outer_parentheses("()()".to_string()), "");
}
