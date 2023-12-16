fn is_anagram(s: String, t: String) -> bool {
    fn f(s: &str) -> Vec<usize> {
        s.bytes().fold(vec![0; 26], |mut acc, b| {
            acc[(b - b'a') as usize] += 1;
            acc
        })
    }

    f(&s) == f(&t)
}

fn main() {
    println!(
        "ret={}",
        is_anagram("anagram".to_string(), "nagaram".to_string())
    );
}

#[test]
fn test() {
    assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    assert!(!is_anagram("rat".to_string(), "cat".to_string()));
}
