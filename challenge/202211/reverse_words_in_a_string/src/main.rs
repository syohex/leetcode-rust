fn reverse_words(s: String) -> String {
    let words: Vec<&str> = s.split(" ").filter(|&s| !s.is_empty()).collect();

    words
        .into_iter()
        .rev()
        .fold(vec![], |mut acc, s| {
            acc.push(s);
            acc
        })
        .join(" ")
}

fn main() {
    let s = "the sky is blue".to_string();
    let ret = reverse_words(s);
    println!("ret={ret}");
}

#[test]
fn test_reverse_words() {
    {
        let s = "the sky is blue".to_string();
        let expected = "blue is sky the";
        let ret = reverse_words(s);
        assert_eq!(ret, expected);
    }
    {
        let s = " hello world   ".to_string();
        let expected = "world hello";
        let ret = reverse_words(s);
        assert_eq!(ret, expected);
    }
    {
        let s = "a good     example".to_string();
        let expected = "example good a";
        let ret = reverse_words(s);
        assert_eq!(ret, expected);
    }
    {
        let s = "apple".to_string();
        let expected = "apple";
        let ret = reverse_words(s);
        assert_eq!(ret, expected);
    }
}
