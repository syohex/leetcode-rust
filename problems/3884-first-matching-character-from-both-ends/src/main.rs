fn first_matching_index(s: String) -> i32 {
    let cs: Vec<_> = s.chars().collect();
    let len = cs.len();
    let limit = len / 2;

    for i in 0..=limit {
        if cs[i] == cs[len - i - 1] {
            return i as i32;
        }
    }
    -1
}

fn main() {
    let ret = first_matching_index("abcacbd".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(first_matching_index("abcacbd".to_string()), 1);
    assert_eq!(first_matching_index("abc".to_string()), 1);
    assert_eq!(first_matching_index("abcdab".to_string()), -1);
}
