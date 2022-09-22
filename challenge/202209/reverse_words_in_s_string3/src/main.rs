fn reverse_words(s: String) -> String {
    s.split_whitespace()
        .into_iter()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let s = "Let's take LeetCode contest".to_string();
    let ret = reverse_words(s);
    println!("ret={ret}");
}

#[test]
fn test_reverse_words() {
    {
        let s = "Let's take LeetCode contest".to_string();
        let expected = "s'teL ekat edoCteeL tsetnoc".to_string();
        let ret = reverse_words(s);
        assert_eq!(ret, expected);
    }
    {
        let s = "God Ding".to_string();
        let expected = "doG gniD".to_string();
        let ret = reverse_words(s);
        assert_eq!(ret, expected);
    }
}
