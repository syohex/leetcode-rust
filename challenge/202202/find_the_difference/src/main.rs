fn find_the_difference(s: String, t: String) -> char {
    use std::collections::HashMap;

    let hs = s.chars().fold(HashMap::new(), |mut h, c| {
        *h.entry(c).or_insert(0) += 1;
        h
    });
    let ht = t.chars().fold(HashMap::new(), |mut h, c| {
        *h.entry(c).or_insert(0) += 1;
        h
    });

    for (k, v) in ht {
        if let Some(&n) = hs.get(&k) {
            if n != v {
                return k;
            }
        } else {
            return k;
        }
    }

    panic!("never reach here");
}

fn main() {
    let s = "abcd".to_string();
    let t = "abcde".to_string();
    let ret = find_the_difference(s, t);
    println!("ret={ret}");
}

#[test]
fn test_find_the_difference() {
    {
        let s = "abcd".to_string();
        let t = "abcde".to_string();
        assert_eq!(find_the_difference(s, t), 'e')
    }
    {
        let s = "".to_string();
        let t = "y".to_string();
        assert_eq!(find_the_difference(s, t), 'y')
    }
}
