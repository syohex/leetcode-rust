fn number_of_substrings(s: String) -> i64 {
    use std::collections::HashMap;

    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        if let Some(v) = h.get_mut(&c) {
            v.push(i);
        } else {
            h.insert(c, vec![i]);
        }
    }

    h.into_iter().fold(0, |acc, (_, v)| {
        if v.len() == 1 {
            acc + 1
        } else {
            let n = (1 + v.len()) * v.len() / 2;
            acc + n as i64
        }
    })
}

fn main() {
    let ret = number_of_substrings("abacad".to_string());
    println!("ret={ret}");
}

#[test]
fn test_number_of_substrings() {
    assert_eq!(number_of_substrings("abcba".to_string()), 7);
    assert_eq!(number_of_substrings("abacad".to_string()), 9);
    assert_eq!(number_of_substrings("a".to_string()), 1);
}
