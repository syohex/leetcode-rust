fn max_depth(s: String) -> i32 {
    let mut ret = 0;
    let mut depth = 0;
    for c in s.chars() {
        match c {
            '(' => {
                depth += 1;
                ret = std::cmp::max(ret, depth);
            }
            ')' => {
                depth -= 1;
            }
            _ => (),
        }
    }

    ret
}

fn main() {
    let ret = max_depth("(1+(2*3)+((8)/4))+1".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    assert_eq!(max_depth("(1)+((2))+(((3)))".to_string()), 3);
    assert_eq!(max_depth("()(())((()()))".to_string()), 3);
}
