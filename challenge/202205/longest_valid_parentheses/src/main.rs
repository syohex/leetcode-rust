fn longest_valid_parentheses(s: String) -> i32 {
    let mut stack = vec![-1];
    let mut ret = 0;

    for (i, c) in s.chars().enumerate() {
        let i = i as i32;
        if c == '(' {
            stack.push(i);
        } else {
            stack.pop();

            if let Some(pos) = stack.last() {
                ret = std::cmp::max(ret, i - pos);
            } else {
                stack.push(i);
            }
        }
    }

    ret
}
fn main() {
    let ret = longest_valid_parentheses("()(())".to_string());
    println!("ret={ret}");
}

#[test]
fn test_longest_valid_parentheses() {
    assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    assert_eq!(longest_valid_parentheses("()(())".to_string()), 6);
    assert_eq!(longest_valid_parentheses("()(()".to_string()), 2);
    assert_eq!(longest_valid_parentheses("".to_string()), 0);
}
