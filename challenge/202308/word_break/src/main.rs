fn word_break(s: String, word_dict: Vec<String>) -> bool {
    use std::collections::{HashSet, VecDeque};

    let words: HashSet<&str> = word_dict.iter().map(|s| s as &str).collect();
    let mut checked = vec![false; s.len()];
    let mut q = VecDeque::new();
    q.push_back(0usize);

    while let Some(pos) = q.pop_front() {
        if pos == s.len() {
            return true;
        }

        for i in pos..s.len() {
            if checked[i] {
                continue;
            }

            let substr = &s[pos..=i];
            if words.contains(&substr) {
                q.push_back(i + 1);
                checked[i] = true;
            }
        }
    }

    false
}

fn main() {
    let s = "leetcode".to_string();
    let word_list = vec!["leet".to_string(), "code".to_string()];
    println!("ret={}", word_break(s, word_list));
}

#[test]
fn test_word_break() {
    {
        let s = "leetcode".to_string();
        let word_list = vec!["leet".to_string(), "code".to_string()];
        assert!(word_break(s, word_list));
    }
    {
        let s = "applepenapple".to_string();
        let word_list = vec!["apple".to_string(), "pen".to_string()];
        assert!(word_break(s, word_list));
    }
    {
        let s = "catsandog".to_string();
        let word_list = vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string(),
        ];
        assert!(!word_break(s, word_list));
    }
}
