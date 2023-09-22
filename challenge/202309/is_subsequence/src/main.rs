fn is_subsequence(s: String, t: String) -> bool {
    let ts: Vec<char> = t.chars().collect();

    let mut i = 0usize;
    'outer: for c in s.chars() {
        while i < t.len() {
            if c == ts[i] {
                i += 1;
                continue 'outer;
            }

            i += 1;
        }

        return false;
    }

    true
}

fn main() {
    let s = "abc".to_string();
    let t = "ahbgdcc".to_string();
    let ret = is_subsequence(s, t);
    println!("ret={ret}");
}

#[test]
fn test_is_subsequence() {
    {
        let s = "abc".to_string();
        let t = "ahbgdcc".to_string();
        assert!(is_subsequence(s, t));
    }
    {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        assert!(!is_subsequence(s, t));
    }
    {
        let s = "ab".to_string();
        let t = "baab".to_string();
        assert!(is_subsequence(s, t));
    }
    {
        let s = "aaaaaa".to_string();
        let t = "bbaaaa".to_string();
        assert!(!is_subsequence(s, t));
    }
}
