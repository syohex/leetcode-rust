fn longest_valid_parentheses(s: String) -> i32 {
    let mut ret = 0;
    let mut start_positions = vec![-1];

    for (i, c) in s.chars().enumerate() {
        let i = i as i32;
        if c == '(' {
            start_positions.push(i);
        } else {
            start_positions.pop();

            if let Some(p) = start_positions.last() {
                ret = std::cmp::max(ret, i - *p);
            } else {
                start_positions.push(i);
            }
        }
    }

    ret
}

fn main() {
    let ret = longest_valid_parentheses("()(()".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    assert_eq!(longest_valid_parentheses("".to_string()), 0);
    assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
}
