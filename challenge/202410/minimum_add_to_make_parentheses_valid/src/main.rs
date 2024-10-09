fn min_add_to_make_valid(s: String) -> i32 {
    let mut opens = 0;
    let mut unbalanced = 0;

    for c in s.chars() {
        if c == '(' {
            opens += 1;
        } else {
            if opens > 0 {
                opens -= 1;
            } else {
                unbalanced += 1;
            }
        }
    }

    unbalanced + opens
}

fn main() {
    let ret = min_add_to_make_valid("())".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_add_to_make_valid("())".to_string()), 1);
    assert_eq!(min_add_to_make_valid("(((".to_string()), 3);
    assert_eq!(min_add_to_make_valid("((()))".to_string()), 0);
    assert_eq!(min_add_to_make_valid("()()()".to_string()), 0);
}
