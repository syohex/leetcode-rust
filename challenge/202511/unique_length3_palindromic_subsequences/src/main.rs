fn count_palindromic_subsequence(s: String) -> i32 {
    use std::collections::HashSet;

    let len = s.len();
    let chars: HashSet<_> = s.chars().collect();

    let mut ret = 0;
    for c in chars {
        let mut start = len;
        let mut end = len;

        for (i, d) in s.chars().enumerate() {
            if d == c {
                if start == len {
                    start = i;
                } else {
                    end = i;
                }
            }
        }

        if end == len {
            continue;
        }

        let mut checked = HashSet::new();
        for d in s.chars().take(end).skip(start + 1) {
            checked.insert(d);
        }

        ret += checked.len() as i32;
    }

    ret
}

fn main() {
    let ret = count_palindromic_subsequence("bbcbaba".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_palindromic_subsequence("aabca".to_string()), 3);
    assert_eq!(count_palindromic_subsequence("abc".to_string()), 0);
    assert_eq!(count_palindromic_subsequence("bbcbaba".to_string()), 4);
}
