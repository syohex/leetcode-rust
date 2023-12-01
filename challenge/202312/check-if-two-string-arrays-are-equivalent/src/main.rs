fn array_strings_are_equal(words1: Vec<String>, words2: Vec<String>) -> bool {
    let w1 = words1.join("");
    let w2 = words2.join("");
    w1 == w2
}

fn main() {
    let words1 = vec!["ab".to_string(), "c".to_string()];
    let words2 = vec!["a".to_string(), "bc".to_string()];
    let ret = array_strings_are_equal(words1, words2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let words1 = vec!["ab".to_string(), "c".to_string()];
        let words2 = vec!["a".to_string(), "bc".to_string()];
        let ret = array_strings_are_equal(words1, words2);
        assert!(ret);
    }
    {
        let words1 = vec!["a".to_string(), "cb".to_string()];
        let words2 = vec!["ab".to_string(), "c".to_string()];
        let ret = array_strings_are_equal(words1, words2);
        assert!(!ret);
    }
    {
        let words1 = vec!["ab".to_string(), "c".to_string(), "defg".to_string()];
        let words2 = vec!["abcdefg".to_string()];
        let ret = array_strings_are_equal(words1, words2);
        assert!(ret);
    }
}
