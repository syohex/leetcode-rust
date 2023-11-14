fn count_palindromic_subsequence(s: String) -> i32 {
    use std::collections::HashSet;

    let v: Vec<char> = s.chars().collect();
    let mut cs: HashSet<char> = HashSet::new();
    for c in v.iter() {
        cs.insert(*c);
    }

    let mut ret = 0;
    for c in cs.into_iter() {
        let first = v.iter().position(|&a| a == c).unwrap();
        let last = v.iter().rposition(|&a| a == c).unwrap();

        let mut chars: HashSet<char> = HashSet::new();
        for i in first + 1..last {
            chars.insert(v[i]);
        }

        ret += chars.len();
    }

    ret as i32
}

fn main() {
    let s = "aabca".to_string();
    let ret = count_palindromic_subsequence(s);
    println!("ret={ret}");
}

#[test]
fn test_count_palindromic_subsequence() {
    {
        let s = "aabca".to_string();
        let ret = count_palindromic_subsequence(s);
        assert_eq!(ret, 3);
    }
    {
        let s = "adc".to_string();
        let ret = count_palindromic_subsequence(s);
        assert_eq!(ret, 0);
    }
    {
        let s = "bbcbaba".to_string();
        let ret = count_palindromic_subsequence(s);
        assert_eq!(ret, 4);
    }
}
