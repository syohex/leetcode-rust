fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    use std::cmp::max;
    use std::collections::HashMap;
    let f = |s: &String| -> HashMap<char, i32> {
        let mut h = HashMap::new();
        for c in s.chars() {
            *h.entry(c).or_insert(0) += 1;
        }

        h
    };

    let mut subword_table = HashMap::new();
    for word in &b {
        let h = f(word);
        for (c, n) in h.iter() {
            if let Some(n2) = subword_table.get_mut(c) {
                *n2 = max(*n2, *n);
            } else {
                subword_table.insert(*c, *n);
            }
        }
    }

    let mut ret = vec![];
    for word in &a {
        let h = f(word);
        let mut ok = true;
        for (c, n) in subword_table.iter() {
            if let Some(n2) = h.get(c) {
                if *n2 < *n {
                    ok = false;
                    break;
                }
            } else {
                ok = false;
                break;
            }
        }

        if ok {
            ret.push(word.clone());
        }
    }
    ret
}

fn main() {
    let a = vec![
        "amazon".to_string(),
        "apple".to_string(),
        "facebook".to_string(),
        "google".to_string(),
        "leetcode".to_string(),
    ];
    let b = vec!["e".to_string(), "o".to_string()];

    let ret = word_subsets(a, b);
    println!("ret={:?}", ret);
}

#[test]
fn test_word_subsets() {
    {
        let a = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let b = vec!["e".to_string(), "o".to_string()];
        let expected = vec![
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];

        let ret = word_subsets(a, b);
        assert_eq!(ret, expected);
    }

    {
        let a = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let b = vec!["l".to_string(), "e".to_string()];
        let expected = vec![
            "apple".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];

        let ret = word_subsets(a, b);
        assert_eq!(ret, expected);
    }
    {
        let a = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let b = vec!["e".to_string(), "oo".to_string()];
        let expected = vec!["facebook".to_string(), "google".to_string()];

        let ret = word_subsets(a, b);
        assert_eq!(ret, expected);
    }
    {
        let a = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let b = vec!["lo".to_string(), "eo".to_string()];
        let expected = vec!["google".to_string(), "leetcode".to_string()];

        let ret = word_subsets(a, b);
        assert_eq!(ret, expected);
    }
    {
        let a = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let b = vec!["ec".to_string(), "oc".to_string(), "ceo".to_string()];
        let expected = vec!["facebook".to_string(), "leetcode".to_string()];

        let ret = word_subsets(a, b);
        assert_eq!(ret, expected);
    }
}
