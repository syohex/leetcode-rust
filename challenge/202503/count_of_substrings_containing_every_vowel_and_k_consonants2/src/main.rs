fn count_of_substrings(word: String, k: i32) -> i64 {
    fn f(cs: &[char], k: i32) -> i64 {
        use std::collections::HashMap;

        let mut h = HashMap::new();
        for c in ['a', 'e', 'i', 'o', 'u'] {
            h.insert(c, 0);
        }

        let len = cs.len();
        let mut left = 0;
        let mut right = 0;

        let mut ret = 0i64;
        let mut consonants = 0;
        while right < len {
            if let Some(v) = h.get_mut(&cs[right]) {
                *v += 1;
            } else {
                consonants += 1;
            }

            while consonants >= k && h.values().all(|&v| v >= 1) {
                ret += (len - right) as i64;
                if let Some(v) = h.get_mut(&cs[left]) {
                    *v -= 1;
                } else {
                    consonants -= 1;
                }
                left += 1;
            }

            right += 1;
        }
        ret
    }

    let cs: Vec<_> = word.chars().collect();
    f(&cs, k) - f(&cs, k + 1)
}

fn main() {
    let ret = count_of_substrings("ieaouqqieaouqq".to_string(), 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let word = "aoaiuefi".to_string();
        let ret = count_of_substrings(word, 1);
        assert_eq!(ret, 4);
    }
    {
        let word = "aeeieoua".to_string();
        let ret = count_of_substrings(word, 0);
        assert_eq!(ret, 5);
    }
    {
        let word = "aeioqq".to_string();
        let ret = count_of_substrings(word, 1);
        assert_eq!(ret, 0);
    }
    {
        let word = "aeiou".to_string();
        let ret = count_of_substrings(word, 0);
        assert_eq!(ret, 1);
    }
    {
        let word = "ieaouqqieaouqq".to_string();
        let ret = count_of_substrings(word, 1);
        assert_eq!(ret, 3);
    }
    {
        let word = "iqeaouqi".to_string();
        let ret = count_of_substrings(word, 2);
        assert_eq!(ret, 3);
    }
}
