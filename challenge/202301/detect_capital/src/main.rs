fn detect_capital_use(word: String) -> bool {
    let cs: Vec<char> = word.chars().collect();
    if cs[0].is_ascii_lowercase() {
        cs.iter().all(|c| c.is_ascii_lowercase())
    } else {
        cs.iter().skip(1).all(|c| c.is_ascii_uppercase())
            || cs.iter().skip(1).all(|c| c.is_ascii_lowercase())
    }
}

fn main() {
    let word = "USA".to_string();
    let ret = detect_capital_use(word);
    println!("ret={ret}");
}

#[test]
fn test_detect_capital_use() {
    {
        let word = "USA".to_string();
        let ret = detect_capital_use(word);
        assert!(ret);
    }
    {
        let word = "Google".to_string();
        let ret = detect_capital_use(word);
        assert!(ret);
    }
    {
        let word = "FlaG".to_string();
        let ret = detect_capital_use(word);
        assert!(!ret);
    }
    {
        let word = "g".to_string();
        let ret = detect_capital_use(word);
        assert!(ret);
    }
}
