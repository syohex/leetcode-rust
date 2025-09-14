fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    use std::collections::{HashMap, HashSet};

    fn to_no_vowel(s: &str) -> String {
        let mut ret = String::new();
        for c in s.chars() {
            let d = match c {
                'a' | 'e' | 'i' | 'o' | 'u' => '*',
                _ => c,
            };
            ret.push(d);
        }

        ret
    }

    let mut origs = HashSet::new();
    let mut lower_cases = HashMap::new();
    let mut no_vowels = HashMap::new();

    for w in wordlist {
        origs.insert(w.clone());

        let lower = w.to_lowercase();
        lower_cases.entry(lower).or_insert(w.clone());

        let no_vowel = to_no_vowel(&w.to_lowercase());
        no_vowels.entry(no_vowel).or_insert(w.clone());
    }

    let mut ret = vec![];
    for q in queries {
        if origs.contains(&q) {
            ret.push(q);
            continue;
        }

        let lower = q.to_lowercase();
        if let Some(v) = lower_cases.get(&lower) {
            ret.push(v.clone());
            continue;
        }

        let no_vowel = to_no_vowel(&lower);
        if let Some(v) = no_vowels.get(&no_vowel) {
            ret.push(v.clone());
            continue;
        }

        ret.push("".to_string());
    }

    ret
}

fn main() {
    let wordlist = vec![
        "KiTe".to_string(),
        "kite".to_string(),
        "hare".to_string(),
        "Hare".to_string(),
    ];
    let queries = vec![
        "kite".to_string(),
        "Kite".to_string(),
        "KiTe".to_string(),
        "Hare".to_string(),
        "HARE".to_string(),
        "Hear".to_string(),
        "hear".to_string(),
        "keti".to_string(),
        "keet".to_string(),
        "keto".to_string(),
    ];
    let ret = spellchecker(wordlist, queries);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let wordlist = vec![
            "KiTe".to_string(),
            "kite".to_string(),
            "hare".to_string(),
            "Hare".to_string(),
        ];
        let queries = vec![
            "kite".to_string(),
            "Kite".to_string(),
            "KiTe".to_string(),
            "Hare".to_string(),
            "HARE".to_string(),
            "Hear".to_string(),
            "hear".to_string(),
            "keti".to_string(),
            "keet".to_string(),
            "keto".to_string(),
        ];
        let expected = vec![
            "kite".to_string(),
            "KiTe".to_string(),
            "KiTe".to_string(),
            "Hare".to_string(),
            "hare".to_string(),
            "".to_string(),
            "".to_string(),
            "KiTe".to_string(),
            "".to_string(),
            "KiTe".to_string(),
        ];
        let ret = spellchecker(wordlist, queries);
        assert_eq!(ret, expected);
    }
    {
        let wordlist = vec!["yellow".to_string()];
        let queries = vec!["YellOw".to_string()];
        let ret = spellchecker(wordlist, queries);
        assert_eq!(ret, vec!["yellow".to_string()]);
    }
}
