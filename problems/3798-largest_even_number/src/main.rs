fn largest_even(s: String) -> String {
    let cs: Vec<_> = s.chars().collect();
    if let Some(p) = cs.iter().rposition(|c| *c == '2') {
        cs.into_iter().take(p + 1).collect()
    } else {
        String::new()
    }
}

fn main() {
    let ret = largest_even("1112".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(largest_even("1112".to_string()), "1112");
    assert_eq!(largest_even("221".to_string()), "22");
    assert_eq!(largest_even("1".to_string()), "");
}
