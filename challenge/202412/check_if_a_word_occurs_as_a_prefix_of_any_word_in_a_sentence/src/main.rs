fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
    if let Some(pos) = sentence
        .split_whitespace()
        .position(|s| s.starts_with(&search_word))
    {
        pos as i32 + 1
    } else {
        -1
    }
}

fn main() {
    let sentence = "i love eating burger".to_string();
    let search_word = "burg".to_string();
    let ret = is_prefix_of_word(sentence, search_word);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let sentence = "i love eating burger".to_string();
        let search_word = "burg".to_string();
        let ret = is_prefix_of_word(sentence, search_word);
        assert_eq!(ret, 4);
    }
    {
        let sentence = "this problem is an easy problem".to_string();
        let search_word = "pro".to_string();
        let ret = is_prefix_of_word(sentence, search_word);
        assert_eq!(ret, 2);
    }
    {
        let sentence = "i am tired".to_string();
        let search_word = "you".to_string();
        let ret = is_prefix_of_word(sentence, search_word);
        assert_eq!(ret, -1);
    }
}
