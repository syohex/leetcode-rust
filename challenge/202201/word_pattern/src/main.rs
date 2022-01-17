fn word_pattern(pattern: String, s: String) -> bool {
    let words: Vec<&str> = s.split(' ').collect();
    if pattern.len() != words.len() {
        return false;
    }

    let cs: Vec<char> = pattern.chars().collect();
    let mut checked = vec![false; words.len()];
    for i in 0..words.len() {
        if checked[i] {
            continue;
        }

        for j in i + 1..words.len() {
            if words[i] == words[j] {
                if cs[i] != cs[j] {
                    return false;
                }
            } else if cs[i] == cs[j] {
                if words[i] != words[j] {
                    return false;
                }
            } else {
                continue;
            }

            checked[j] = true;
        }
    }

    true
}

fn main() {
    let ret = word_pattern("abba".to_string(), "dog cat cat dog".to_string());
    println!("ret={ret}");
}

#[test]
fn test_word_pattern() {
    assert!(word_pattern(
        "abba".to_string(),
        "dog cat cat dog".to_string()
    ));
    assert!(!word_pattern(
        "abba".to_string(),
        "dog cat cat fish".to_string()
    ));
    assert!(!word_pattern(
        "aaaa".to_string(),
        "dog cat cat dog".to_string()
    ));
    assert!(!word_pattern(
        "aaaa".to_string(),
        "dog cat cat dog".to_string()
    ));
    assert!(!word_pattern(
        "a".to_string(),
        "dog cat cat dog".to_string()
    ));
    assert!(!word_pattern(
        "abba".to_string(),
        "dog dog dog dog".to_string()
    ));
    assert!(!word_pattern(
        "abba".to_string(),
        "fish whoops helloworld fish".to_string()
    ));
}
