fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut ret = vec![];
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i != j && words[j].contains(&words[i]) {
                ret.push(words[i].clone());
                break;
            }
        }
    }
    ret
}

fn main() {
    let words = vec![
        "mass".to_string(),
        "as".to_string(),
        "hero".to_string(),
        "superhero".to_string(),
    ];
    let ret = string_matching(words);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let words = vec![
            "leetcoder".to_string(),
            "leetcode".to_string(),
            "od".to_string(),
            "hamlet".to_string(),
            "am".to_string(),
        ];
        let expected = vec!["leetcode".to_string(), "od".to_string(), "am".to_string()];
        let ret = string_matching(words);
        assert_eq!(ret, expected);
    }
    {
        let words = vec![
            "mass".to_string(),
            "as".to_string(),
            "hero".to_string(),
            "superhero".to_string(),
        ];
        let expected = vec!["as".to_string(), "hero".to_string()];
        let ret = string_matching(words);
        assert_eq!(ret, expected);
    }
    {
        let words = vec!["leetcode".to_string(), "et".to_string(), "code".to_string()];
        let expected = vec!["et".to_string(), "code".to_string()];
        let ret = string_matching(words);
        assert_eq!(ret, expected);
    }
    {
        let words = vec!["blue".to_string(), "green".to_string(), "bu".to_string()];
        let ret = string_matching(words);
        assert!(ret.is_empty());
    }
}
