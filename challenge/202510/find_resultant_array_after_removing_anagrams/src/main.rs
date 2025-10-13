fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    fn f(s: &str) -> Vec<i32> {
        let mut ret = vec![0; 26];
        for b in s.bytes() {
            ret[(b - b'a') as usize] += 1;
        }
        ret
    }

    let mut ret = vec![];
    let mut prev = vec![-1; 26];
    for w in words {
        let freq = f(&w);
        if freq != prev {
            ret.push(w);
            prev = freq;
        }
    }

    ret
}

fn main() {
    let words = vec![
        "abba".to_string(),
        "baba".to_string(),
        "bbaa".to_string(),
        "cd".to_string(),
        "cd".to_string(),
    ];
    let ret = remove_anagrams(words);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let words = vec![
            "abba".to_string(),
            "baba".to_string(),
            "bbaa".to_string(),
            "cd".to_string(),
            "cd".to_string(),
        ];
        let expected = vec!["abba".to_string(), "cd".to_string()];
        let ret = remove_anagrams(words);
        assert_eq!(ret, expected);
    }
    {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];
        let expected = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];
        let ret = remove_anagrams(words);
        assert_eq!(ret, expected);
    }
}
