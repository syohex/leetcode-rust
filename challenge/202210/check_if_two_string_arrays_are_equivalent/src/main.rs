fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let w1 = word1.join("");
    let w2 = word2.join("");
    w1 == w2
}

fn main() {
    let word1 = vec!["ab".to_string(), "c".to_string()];
    let word2 = vec!["a".to_string(), "bc".to_string()];
    let ret = array_strings_are_equal(word1, word2);
    println!("ret={ret}");
}

#[test]
fn test_array_strings_are_equal() {
    {
        let word1 = vec!["ab".to_string(), "c".to_string()];
        let word2 = vec!["a".to_string(), "bc".to_string()];
        let ret = array_strings_are_equal(word1, word2);
        assert!(ret);
    }
    {
        let word1 = vec!["a".to_string(), "cb".to_string()];
        let word2 = vec!["ab".to_string(), "c".to_string()];
        let ret = array_strings_are_equal(word1, word2);
        assert!(!ret);
    }
    {
        let word1 = vec!["abc".to_string(), "d".to_string(), "defg".to_string()];
        let word2 = vec!["abcddefg".to_string()];
        let ret = array_strings_are_equal(word1, word2);
        assert!(ret);
    }
}
