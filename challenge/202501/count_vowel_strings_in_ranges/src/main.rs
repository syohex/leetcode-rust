fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a'|'e'|'i'|'o'|'u')
    }

    let mut acc = vec![];
    acc.push(0);

    for word in words {
        let s = word.chars().next().unwrap();
        let e = word.chars().last().unwrap();
        let prev = acc.last().unwrap();
        if is_vowel(s) && is_vowel(e) {
            acc.push(*prev + 1);
        } else {
            acc.push(*prev);
        }
    }

    let mut ret = vec![];
    for q in queries {
        ret.push(acc[q[1] as usize + 1] - acc[q[0] as usize]);
    }

    ret
}

fn main() {
    let words = vec![
        "aba".to_string(),
        "bcb".to_string(),
        "ece".to_string(),
        "aa".to_string(),
        "e".to_string(),
    ];
    let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
    let ret = vowel_strings(words, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let words = vec![
            "aba".to_string(),
            "bcb".to_string(),
            "ece".to_string(),
            "aa".to_string(),
            "e".to_string(),
        ];
        let queries = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
        let expected = vec![2, 3, 0];
        let ret = vowel_strings(words, queries);
        assert_eq!(ret, expected);
    }
    {
        let words = vec!["a".to_string(), "e".to_string(), "i".to_string()];
        let queries = vec![vec![0, 2], vec![0, 1], vec![2, 2]];
        let expected = vec![3, 2, 1];
        let ret = vowel_strings(words, queries);
        assert_eq!(ret, expected);
    }
}
