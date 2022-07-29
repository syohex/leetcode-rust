fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
    fn f(word: &str) -> Vec<usize> {
        use std::collections::HashMap;

        let mut h = HashMap::new();
        let mut ret = vec![];
        for (i, c) in word.chars().enumerate() {
            if let Some(&pos) = h.get(&c) {
                ret.push(pos);
            } else {
                h.insert(c, i);
                ret.push(i);
            }
        }

        ret
    }

    let pv = f(&pattern);

    let mut ret = vec![];
    for word in words {
        if f(&word) == pv {
            ret.push(word);
        }
    }

    ret
}

fn main() {
    let words = vec![
        "abc".to_string(),
        "deq".to_string(),
        "mee".to_string(),
        "aqq".to_string(),
        "dkd".to_string(),
        "ccc".to_string(),
    ];
    let pattern = "abb".to_string();

    let ret = find_and_replace_pattern(words, pattern);
    println!("ret={:?}", ret);
}

#[test]
fn test_find_and_replace_pattern() {
    {
        let words = vec![
            "ktittgzawn".to_string(),
            "dgphvfjniv".to_string(),
            "gceqobzmis".to_string(),
            "alrztxdlah".to_string(),
            "jijuevoioe".to_string(),
            "mawiizpkub".to_string(),
            "onwpmnujos".to_string(),
            "zszkptjgzj".to_string(),
            "zwfvzhrucv".to_string(),
            "isyaphcszn".to_string(),
        ];
        let pattern = "zdqmjnczma".to_string();

        let ret = find_and_replace_pattern(words, pattern);
        assert!(ret.is_empty());
    }
    {
        let words = vec![
            "abc".to_string(),
            "deq".to_string(),
            "mee".to_string(),
            "aqq".to_string(),
            "dkd".to_string(),
            "ccc".to_string(),
        ];
        let pattern = "abb".to_string();
        let expected = vec!["mee".to_string(), "aqq".to_string()];

        let ret = find_and_replace_pattern(words, pattern);
        assert_eq!(ret, expected);
    }
    {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let pattern = "a".to_string();
        let expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        let ret = find_and_replace_pattern(words, pattern);
        assert_eq!(ret, expected);
    }
    {
        let words = vec![
            "qqcojjumwp".to_string(),
            "mqidrqudxu".to_string(),
            "xwrvnueult".to_string(),
            "lubbymxyro".to_string(),
            "fcvxuskhcl".to_string(),
        ];
        let pattern = "rdzkpkbmda".to_string();

        let ret = find_and_replace_pattern(words, pattern);
        assert!(ret.is_empty());
    }
    {
        let words = vec![
            "abc".to_string(),
            "cba".to_string(),
            "xyx".to_string(),
            "yxx".to_string(),
            "yyx".to_string(),
        ];
        let pattern = "abc".to_string();
        let expected = vec!["abc".to_string(), "cba".to_string()];

        let ret = find_and_replace_pattern(words, pattern);
        assert_eq!(ret, expected);
    }
}
