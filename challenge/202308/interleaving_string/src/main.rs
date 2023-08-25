fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    use std::collections::HashSet;
    fn f(
        n1: usize,
        s1: &Vec<char>,
        n2: usize,
        s2: &Vec<char>,
        n3: usize,
        s3: &Vec<char>,
        cache: &mut HashSet<(usize, usize, usize)>,
    ) -> bool {
        if n3 == s3.len() {
            return n1 == s1.len() && n2 == s2.len();
        }

        let key = (n1, n2, n3);
        if cache.contains(&key) {
            return false;
        }

        if n1 < s1.len() && s1[n1] == s3[n3] {
            if f(n1 + 1, s1, n2, s2, n3 + 1, s3, cache) {
                return true;
            }
        }
        if n2 < s2.len() && s2[n2] == s3[n3] {
            if f(n1, s1, n2 + 1, s2, n3 + 1, s3, cache) {
                return true;
            }
        }

        cache.insert(key);
        false
    }

    let s1 = s1.chars().collect();
    let s2 = s2.chars().collect();
    let s3 = s3.chars().collect();
    let mut cache = HashSet::new();
    f(0, &s1, 0, &s2, 0, &s3, &mut cache)
}

fn main() {
    let s1 = "aabcc".to_string();
    let s2 = "dbbca".to_string();
    let s3 = "aadbbcbcac".to_string();
    let ret = is_interleave(s1, s2, s3);
    println!("ret={ret}");
}

#[test]
fn test_is_interleave() {
    {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        let ret = is_interleave(s1, s2, s3);
        assert!(ret);
    }
    {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadnbbbaccc".to_string();
        let ret = is_interleave(s1, s2, s3);
        assert!(!ret);
    }
    {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();
        let ret = is_interleave(s1, s2, s3);
        assert!(ret);
    }
    {
        let s1 = "a".to_string();
        let s2 = "b".to_string();
        let s3 = "a".to_string();
        let ret = is_interleave(s1, s2, s3);
        assert!(!ret);
    }
}
