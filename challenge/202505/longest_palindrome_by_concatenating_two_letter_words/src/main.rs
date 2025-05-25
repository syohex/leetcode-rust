fn longest_palindrome(words: Vec<String>) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for word in words {
        let cs: Vec<_> = word.chars().collect();
        *h.entry((cs[0], cs[1])).or_insert(0) += 1;
    }

    let mut ret = 0;
    let mut has_center = false;
    for (&(c1, c2), &count) in h.iter() {
        if c1 == c2 {
            if count % 2 == 0 {
                ret += 2 * count;
            } else {
                ret += 2 * (count - 1);
                has_center = true;
            }
        } else if c1 < c2 {
            if let Some(&v) = h.get(&(c2, c1)) {
                ret += 4 * std::cmp::min(count, v);
            }
        }
    }

    if has_center {
        ret += 2;
    }

    ret
}

fn main() {
    let words = vec!["lc".to_string(), "cl".to_string(), "gg".to_string()];
    let ret = longest_palindrome(words);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let words = vec!["lc".to_string(), "cl".to_string(), "gg".to_string()];
        let ret = longest_palindrome(words);
        assert_eq!(ret, 6);
    }
    {
        let words = vec![
            "ab".to_string(),
            "ty".to_string(),
            "yt".to_string(),
            "lc".to_string(),
            "cl".to_string(),
            "ab".to_string(),
        ];
        let ret = longest_palindrome(words);
        assert_eq!(ret, 8);
    }
    {
        let words = vec!["cc".to_string(), "ll".to_string(), "xx".to_string()];
        let ret = longest_palindrome(words);
        assert_eq!(ret, 2);
    }
}
