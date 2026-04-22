fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
    let mut ret = vec![];
    for q in queries.into_iter() {
        for d in &dictionary {
            let mut diff = 0;
            for (a, b) in q.chars().zip(d.chars()) {
                if a != b {
                    diff += 1;
                    if diff >= 3 {
                        break;
                    }
                }
            }

            if diff <= 2 {
                ret.push(q);
                break;
            }
        }
    }

    ret
}

fn main() {
    let queries = vec![
        "word".to_string(),
        "note".to_string(),
        "ants".to_string(),
        "wood".to_string(),
    ];
    let dictionary = vec!["wood".to_string(), "joke".to_string(), "moat".to_string()];
    let ret = two_edit_words(queries, dictionary);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let queries = vec![
            "word".to_string(),
            "note".to_string(),
            "ants".to_string(),
            "wood".to_string(),
        ];
        let dictionary = vec!["wood".to_string(), "joke".to_string(), "moat".to_string()];
        let expected = vec!["word".to_string(), "note".to_string(), "wood".to_string()];
        let ret = two_edit_words(queries, dictionary);
        assert_eq!(ret, expected);
    }
    {
        let queries = vec!["yes".to_string()];
        let dictionary = vec!["not".to_string()];
        let ret = two_edit_words(queries, dictionary);
        assert!(ret.is_empty());
    }
}
