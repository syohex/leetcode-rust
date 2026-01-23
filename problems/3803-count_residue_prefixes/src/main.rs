fn residue_prefixes(s: String) -> i32 {
    use std::collections::HashSet;

    let mut checked = HashSet::new();
    let mut ret = 0;
    for (i, c) in s.chars().enumerate() {
        checked.insert(c);
        if (i + 1) % 3 == checked.len() {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let ret = residue_prefixes("abc".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(residue_prefixes("abc".to_string()), 2);
    assert_eq!(residue_prefixes("dd".to_string()), 1);
    assert_eq!(residue_prefixes("bob".to_string()), 2);
    assert_eq!(residue_prefixes("a".to_string()), 1);
    assert_eq!(residue_prefixes("bbbb".to_string()), 2);
}
