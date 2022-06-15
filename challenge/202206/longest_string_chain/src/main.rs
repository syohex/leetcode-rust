fn longest_str_chain(words: Vec<String>) -> i32 {
    use std::collections::HashMap;
    use std::collections::HashSet;

    fn f(word: &str, s: &HashSet<String>, cache: &mut HashMap<String, i32>) -> i32 {
        if let Some(&v) = cache.get(word) {
            return v;
        }

        let len = word.len();
        let mut ret = 1;
        for i in 0..len {
            let t = if i == 0 {
                word[1..].to_string()
            } else if i == len - 1 {
                word[..len - 1].to_string()
            } else {
                [&word[..i], &word[i + 1..]].concat()
            };

            if s.contains(&t) {
                ret = std::cmp::max(ret, 1 + f(&t, s, cache));
            }
        }

        cache.insert(word.to_string(), ret);
        ret
    }

    let s: HashSet<String> = words.iter().map(|word| word.clone()).collect();
    let mut cache = HashMap::new();
    words
        .into_iter()
        .map(|word| f(&word, &s, &mut cache))
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
