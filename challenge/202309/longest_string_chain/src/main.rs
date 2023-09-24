fn longest_str_chain(words: Vec<String>) -> i32 {
    use std::collections::{HashMap, HashSet};

    fn f(word: &str, words: &HashSet<&str>, cache: &mut HashMap<String, i32>) -> i32 {
        if let Some(n) = cache.get(word) {
            return *n;
        }

        let mut ret = 1;
        for i in 0..word.len() {
            let s = if i == 0 {
                word[1..].to_string()
            } else if i == word.len() - 1 {
                word[..i].to_string()
            } else {
                [&word[..i], &word[i + 1..]].concat()
            };

            if words.contains(s.as_str()) {
                ret = std::cmp::max(ret, 1 + f(s.as_str(), words, cache));
            }
        }

        cache.insert(word.to_string(), ret);
        ret
    }

    let words: HashSet<&str> = words.iter().map(|s| s.as_str()).collect();
    let mut cache = HashMap::new();
    words
        .iter()
        .map(|word| f(*word, &words, &mut cache))
        .max()
        .unwrap()
}

fn main() {
    let words = vec![
        "a".to_string(),
        "b".to_string(),
        "ba".to_string(),
        "bca".to_string(),
        "bda".to_string(),
        "bdca".to_string(),
    ];
    let ret = longest_str_chain(words);
    println!("ret={ret}");
}

#[test]
fn test_longest_str_chain() {
    {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "ba".to_string(),
            "bca".to_string(),
            "bda".to_string(),
            "bdca".to_string(),
        ];
        let ret = longest_str_chain(words);
        assert_eq!(ret, 4);
    }
    {
        let words = vec![
            "xbc".to_string(),
            "pcxbcf".to_string(),
            "xb".to_string(),
            "cxbc".to_string(),
            "pcxbc".to_string(),
        ];
        let ret = longest_str_chain(words);
        assert_eq!(ret, 5);
    }
    {
        let words = vec!["abcd".to_string(), "dbqca".to_string()];
        let ret = longest_str_chain(words);
        assert_eq!(ret, 1);
    }
}
