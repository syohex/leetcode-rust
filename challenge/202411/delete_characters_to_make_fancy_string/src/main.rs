fn make_funcy_string(s: String) -> String {
    s.chars()
        .fold((String::new(), '?', '?'), |(mut acc, prev1, prev2), c| {
            if c == prev1 && c == prev2 {
                (acc, c, prev1)
            } else {
                acc.push(c);
                (acc, c, prev1)
            }
        })
        .0
}

fn main() {
    let ret = make_funcy_string("aaabaaaa".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(make_funcy_string("leeetcode".to_string()), "leetcode");
    assert_eq!(make_funcy_string("aaabaaaa".to_string()), "aabaa");
    assert_eq!(make_funcy_string("aab".to_string()), "aab");
}
