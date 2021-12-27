fn first_palindrome(words: Vec<String>) -> String {
    for word in &words {
        let ok = word.chars().zip(word.chars().rev()).all(|(a, b)| a == b);
        if ok {
            return word.clone();
        }
    }

    "".to_string()
}

fn main() {
    let words = vec![
        "abc".to_string(),
        "car".to_string(),
        "ada".to_string(),
        "racecar".to_string(),
        "cool".to_string(),
    ];
    println!("ret={}", first_palindrome(words));
}

#[test]
fn test_first_palindrome() {
    {
        let words = vec![
            "abc".to_string(),
            "car".to_string(),
            "ada".to_string(),
            "racecar".to_string(),
            "cool".to_string(),
        ];
        assert_eq!(first_palindrome(words), "ada".to_string());
    }
    {
        let words = vec!["notapalindrome".to_string(), "racecar".to_string()];
        assert_eq!(first_palindrome(words), "racecar".to_string());
    }
    {
        let words = vec!["def".to_string(), "ghi".to_string()];
        assert_eq!(first_palindrome(words), "".to_string());
    }
}
