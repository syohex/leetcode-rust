fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
    let f = |s: &String| -> i32 { s.bytes().fold(0, |acc, b| acc * 10 + (b - b'a') as i32) };

    f(&first_word) + f(&second_word) == f(&target_word)
}

fn main() {
    let first_word = "acb".to_string();
    let second_word = "cba".to_string();
    let target_word = "cdb".to_string();
    let ret = is_sum_equal(first_word, second_word, target_word);
    println!("ret={}", ret);
}

#[test]
fn test_isu_sum_equal() {
    {
        let first_word = "acb".to_string();
        let second_word = "cba".to_string();
        let target_word = "cdb".to_string();
        assert!(is_sum_equal(first_word, second_word, target_word));
    }
    {
        let first_word = "aaa".to_string();
        let second_word = "a".to_string();
        let target_word = "aab".to_string();
        assert!(!is_sum_equal(first_word, second_word, target_word));
    }
    {
        let first_word = "aaa".to_string();
        let second_word = "a".to_string();
        let target_word = "aaaa".to_string();
        assert!(is_sum_equal(first_word, second_word, target_word));
    }
}
