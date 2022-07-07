fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    use std::collections::HashMap;

    fn f(
        p1: usize,
        s1: &Vec<char>,
        p2: usize,
        s2: &Vec<char>,
        p3: usize,
        s3: &Vec<char>,
        cache: &mut HashMap<(usize, usize, usize), bool>,
    ) -> bool {
        if p3 == s3.len() {
            return p1 == s1.len() && p2 == s2.len();
        }

        let key = (p1, p2, p3);
        if let Some(&b) = cache.get(&key) {
            return b;
        }

        if p1 < s1.len() && s1[p1] == s3[p3] {
            if f(p1 + 1, s1, p2, s2, p3 + 1, s3, cache) {
                cache.insert(key, true);
                return true;
            }
        }

        if p2 < s2.len() && s2[p2] == s3[p3] {
            if f(p1, s1, p2 + 1, s2, p3 + 1, s3, cache) {
                cache.insert(key, true);
                return true;
            }
        }

        cache.insert(key, false);
        false
    }

    let cs1 = s1.chars().collect();
    let cs2 = s2.chars().collect();
    let cs3 = s3.chars().collect();
    let mut cache = HashMap::new();

    f(0, &cs1, 0, &cs2, 0, &cs3, &mut cache)
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

        assert!(is_interleave(s1, s2, s3));
    }
    {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();

        assert!(!is_interleave(s1, s2, s3));
    }
    {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();

        assert!(is_interleave(s1, s2, s3));
    }
    {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "a".to_string();

        assert!(!is_interleave(s1, s2, s3));
    }
    {
        let s1 = "a".to_string();
        let s2 = "b".to_string();
        let s3 = "a".to_string();

        assert!(!is_interleave(s1, s2, s3));
    }
    {
        let s1 = "bbbbbabbbbabaababaaaabbababbaaabbabbaaabaaaaababbbababbbbbabbbbababbabaabababbbaabababababbbaaababaa".to_string();
        let s2 = "babaaaabbababbbabbbbaabaabbaabbbbaabaaabaababaaaabaaabbaaabaaaabaabaabbbbbbbbbbbabaaabbababbabbabaab".to_string();
        let s3 = "babbbabbbaaabbababbbbababaabbabaabaaabbbbabbbaaabbbaaaaabbbbaabbaaabababbaaaaaabababbababaababbababbbababbbbaaaabaabbabbaaaaabbabbaaaabbbaabaaabaababaababbaaabbbbbabbbbaabbabaabbbbabaaabbababbabbabbab".to_string();

        assert!(!is_interleave(s1, s2, s3));
    }
}
