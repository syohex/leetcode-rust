fn find_the_difference(s: String, t: String) -> char {
    use std::collections::HashMap;

    let ms = s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let ts = t.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    for (k, v) in ts.into_iter() {
        if let Some(&count) = ms.get(&k) {
            if count != v {
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
        let ret = find_the_difference(s, t);
        assert_eq!(ret, 'e');
    }
    {
        let s = "".to_string();
        let t = "y".to_string();
        let ret = find_the_difference(s, t);
        assert_eq!(ret, 'y');
    }
}
