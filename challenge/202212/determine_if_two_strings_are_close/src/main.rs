fn close_strings(word1: String, word2: String) -> bool {
    use std::collections::{HashMap, HashSet};

    let mut m1 = HashMap::new();
    for c in word1.chars() {
        *m1.entry(c).or_insert(0) += 1;
    }

    let mut m2 = HashMap::new();
    for c in word2.chars() {
        *m2.entry(c).or_insert(0) += 1;
    }

    let s1: HashSet<char> = m1.keys().map(|c| *c).collect();
    let s2: HashSet<char> = m2.keys().map(|c| *c).collect();
    if s1 != s2 {
        return false;
    }

    let mut freq1 = HashMap::new();
    let mut freq2 = HashMap::new();
    for (v1, v2) in m1.values().zip(m2.values()) {
        *freq1.entry(*v1).or_insert(0) += 1;
        *freq2.entry(*v2).or_insert(0) += 1;
    }

    freq1 == freq2
}

fn main() {
    let word1 = "cabbba".to_string();
    let word2 = "abbccc".to_string();
    let ret = close_strings(word1, word2);
    println!("ret={ret}");
}

#[test]
fn test_close_strings() {
    {
        let word1 = "abc".to_string();
        let word2 = "bca".to_string();
        assert!(close_strings(word1, word2));
    }
    {
        let word1 = "a".to_string();
        let word2 = "aa".to_string();
        assert!(!close_strings(word1, word2));
    }
    {
        let word1 = "cabbba".to_string();
        let word2 = "abbccc".to_string();
        assert!(close_strings(word1, word2));
    }
    {
        let word1 = "uau".to_string();
        let word2 = "ssx".to_string();
        assert!(!close_strings(word1, word2));
    }
}
