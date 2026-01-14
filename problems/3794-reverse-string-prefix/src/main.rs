fn reverse_prefix(s: String, k: i32) -> String {
    let k = k as usize;
    let mut pre: Vec<_> = s.chars().take(k).collect();
    pre.reverse();
    let mut post: Vec<_> = s.chars().skip(k).collect();

    pre.append(&mut post);
    pre.into_iter().collect()
}

fn main() {
    let ret = reverse_prefix("abcd".to_string(), 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(reverse_prefix("abcd".to_string(), 2), "bacd");
    assert_eq!(reverse_prefix("xyz".to_string(), 3), "zyx");
    assert_eq!(reverse_prefix("hey".to_string(), 1), "hey");
}
