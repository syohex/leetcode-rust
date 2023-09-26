fn remove_duplicate_letters(s: String) -> String {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for c in s.chars() {
        *h.entry(c).or_insert(0) += 1;
    }

    let mut v = vec![];
    let mut used = vec![false; 26];

    for c in s.chars() {
        *h.get_mut(&c).unwrap() -= 1;

        let index = c as usize - 'a' as usize;
        if used[index] {
            continue;
        }

        while let Some(&d) = v.last() {
            if c < d && *h.get(&d).unwrap() > 0 {
                let idx = d as usize - 'a' as usize;
                used[idx] = false;
                v.pop();
            } else {
                break;
            }
        }

        used[index] = true;
        v.push(c);
    }

    v.into_iter().collect()
}

fn main() {
    let s = "cbacdcbc".to_string();
    let ret = remove_duplicate_letters(s);
    println!("ret={ret}");
}

#[test]
fn test_remove_duplicate_letters() {
    {
        let s = "bcabc".to_string();
        let ret = remove_duplicate_letters(s);
        assert_eq!(ret, "abc");
    }
    {
        let s = "cbacdcbc".to_string();
        let ret = remove_duplicate_letters(s);
        assert_eq!(ret, "acdb");
    }
    {
        let s = "bbcaac".to_string();
        let ret = remove_duplicate_letters(s);
        assert_eq!(ret, "bac");
    }
    {
        let s = "ecbacba".to_string();
        let ret = remove_duplicate_letters(s);
        assert_eq!(ret, "eacb");
    }
    {
        let s = "abacb".to_string();
        let ret = remove_duplicate_letters(s);
        assert_eq!(ret, "abc");
    }
}
