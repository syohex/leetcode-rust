fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    words
        .into_iter()
        .enumerate()
        .fold(vec![], |mut acc, (i, word)| {
            if let Some(_) = word.chars().position(|c| c == x) {
                acc.push(i as i32);
            }

            acc
        })
}

fn main() {
    let words = vec!["leet".to_string(), "code".to_string()];
    let ret = find_words_containing(words, 'e');
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let words = vec!["leet".to_string(), "code".to_string()];
        let expected = vec![0, 1];
        let ret = find_words_containing(words, 'e');
        assert_eq!(ret, expected);
    }
    {
        let words = vec![
            "abc".to_string(),
            "bcd".to_string(),
            "aaaa".to_string(),
            "cbc".to_string(),
        ];
        let expected = vec![0, 2];
        let ret = find_words_containing(words, 'a');
        assert_eq!(ret, expected);
    }
    {
        let words = vec![
            "abc".to_string(),
            "bcd".to_string(),
            "aaaa".to_string(),
            "cbc".to_string(),
        ];
        let expected = vec![];
        let ret = find_words_containing(words, 'z');
        assert_eq!(ret, expected);
    }
}
