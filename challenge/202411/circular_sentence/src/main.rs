fn is_circular_sentence(sentence: String) -> bool {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    words
        .iter()
        .fold((true, words.last().unwrap()), |(acc, prev), s| {
            if acc {
                match (prev.chars().last(), s.chars().next()) {
                    (Some(c1), Some(c2)) if c1 == c2 => (true, s),
                    _ => (false, s),
                }
            } else {
                (false, s)
            }
        })
        .0
}

fn main() {
    let ret = is_circular_sentence("leetcode exercises sound delightful".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_circular_sentence(
        "leetcode exercises sound delightful".to_string()
    ));
    assert!(is_circular_sentence("eetcode".to_string()));
    assert!(!is_circular_sentence("leetcode".to_string()));
    assert!(!is_circular_sentence("Leetcode is cool".to_string()));
}
