fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    let mut prev = 2;
    let mut ret = vec![];
    for (word, group) in words.into_iter().zip(groups.into_iter()) {
        if group != prev {
            ret.push(word);
        }
        prev = group;
    }

    ret
}

fn main() {
    let words = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ];
    let groups = vec![1, 0, 1, 1];
    let ret = get_longest_subsequence(words, groups);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let words = vec!["e".to_string(), "a".to_string(), "b".to_string()];
        let groups = vec![0, 0, 1];
        let expected = vec!["e".to_string(), "b".to_string()];
        let ret = get_longest_subsequence(words, groups);
        assert_eq!(ret, expected);
    }
    {
        let words = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let groups = vec![1, 0, 1, 1];
        let expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let ret = get_longest_subsequence(words, groups);
        assert_eq!(ret, expected);
    }
}
