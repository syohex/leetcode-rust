fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    fn is_prefix_and_suffix(str1: &str, str2: &str) -> bool {
        str2.starts_with(str1) && str2.ends_with(str1)
    }

    let len = words.len();
    let mut ret = 0;
    for i in 0..len {
        for j in (i + 1)..len {
            if is_prefix_and_suffix(&words[i], &words[j]) {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let words = vec![
        "a".to_string(),
        "aba".to_string(),
        "ababa".to_string(),
        "aa".to_string(),
    ];
    let ret = count_prefix_suffix_pairs(words);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let words = vec![
            "a".to_string(),
            "aba".to_string(),
            "ababa".to_string(),
            "aa".to_string(),
        ];
        let ret = count_prefix_suffix_pairs(words);
        assert_eq!(ret, 4);
    }
    {
        let words = vec![
            "pa".to_string(),
            "papa".to_string(),
            "ma".to_string(),
            "mama".to_string(),
        ];
        let ret = count_prefix_suffix_pairs(words);
        assert_eq!(ret, 2);
    }
    {
        let words = vec!["abab".to_string(), "ab".to_string()];
        let ret = count_prefix_suffix_pairs(words);
        assert_eq!(ret, 0);
    }
}
