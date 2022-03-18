fn remove_duplicate_letters(s: String) -> String {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let mut h = HashMap::new();
    for c in s.chars() {
        *h.entry(c).or_insert(0) += 1;
    }

    let mut ret = vec![];
    let mut checked = HashSet::new();
    for c in s.chars() {
        *h.get_mut(&c).unwrap() -= 1;

        if checked.contains(&c) {
            continue;
        }

        while let Some(&a) = ret.last() {
            if c < a && *h.get(&a).unwrap() > 0 {
                checked.remove(&a);
                ret.pop();
            } else {
                break;
            }
        }

        checked.insert(c);
        ret.push(c);
    }

    ret.into_iter().collect()
}

fn main() {
    let ret = remove_duplicate_letters("cbacdcbc".to_string());
    println!("ret={ret}");
}

#[test]
fn test_remove_duplicate_letters() {
    assert_eq!(
        remove_duplicate_letters("bcabc".to_string()),
        "abc".to_string()
    );
    assert_eq!(
        remove_duplicate_letters("cbacdcbc".to_string()),
        "acdb".to_string()
    );
}
