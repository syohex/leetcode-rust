fn check_if_pangram(sentence: String) -> bool {
    use std::collections::HashSet;

    sentence
        .chars()
        .fold(HashSet::new(), |mut acc, c| {
            acc.insert(c);
            acc
        })
        .len()
        == 26
}

fn main() {
    let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
    let ret = check_if_pangram(sentence);
    println!("ret={ret}");
}

#[test]
fn test_check_if_pangram() {
    {
        let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
        assert!(check_if_pangram(sentence));
    }
    {
        let sentence = "leetcode".to_string();
        assert!(!check_if_pangram(sentence));
    }
}
