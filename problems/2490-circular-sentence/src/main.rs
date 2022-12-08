fn is_circular_sentence(sentence: String) -> bool {
    let v: Vec<char> = sentence.chars().collect();

    for i in 0..v.len() {
        if v[i] == ' ' && v[i - 1] != v[i + 1] {
            return false;
        }
    }

    v[0] == v[v.len() - 1]
}

fn main() {
    let sentence = "leetcode exercises sound delightful".to_string();
    let ret = is_circular_sentence(sentence);
    println!("ret={ret}");
}

#[test]
fn test_is_circular_sentence() {
    {
        let sentence = "leetcode exercises sound delightful".to_string();
        assert!(is_circular_sentence(sentence));
    }
    {
        let sentence = "eetcode".to_string();
        assert!(is_circular_sentence(sentence));
    }
    {
        let sentence = "Leetcode is cool".to_string();
        assert!(!is_circular_sentence(sentence));
    }
}
