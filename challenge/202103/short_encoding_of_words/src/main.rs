fn minimum_length_encoding(words: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let mut rets: HashSet<&str> = HashSet::new();

    for word in &words {
        rets.insert(word);
    }

    let mut ret = 0;
    for &word in &rets {
        if !rets.iter().any(|&s| word != s && s.ends_with(word)) {
            ret += word.len() + 1;
        }
    }

    ret as i32
}

fn main() {
    let ret = minimum_length_encoding(vec![
        "time".to_string(),
        "me".to_string(),
        "bell".to_string(),
    ]);
    println!("ret={}", ret);
}

#[test]
fn test_minimum_length_encoding() {
    assert_eq!(
        minimum_length_encoding(vec![
            "time".to_string(),
            "me".to_string(),
            "bell".to_string()
        ]),
        10
    );
    assert_eq!(minimum_length_encoding(vec!["t".to_string()]), 2);
    assert_eq!(
        minimum_length_encoding(vec![
            "time".to_string(),
            "time".to_string(),
            "time".to_string(),
            "time".to_string(),
        ]),
        5
    );
}
