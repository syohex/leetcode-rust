fn check_strings(s1: String, s2: String) -> bool {
    use std::collections::HashMap;

    let mut evens = HashMap::new();
    let mut odds = HashMap::new();

    for (i, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
        if i % 2 == 0 {
            *evens.entry(c1).or_insert(0) += 1;
            *evens.entry(c2).or_insert(0) -= 1;
        } else {
            *odds.entry(c1).or_insert(0) += 1;
            *odds.entry(c2).or_insert(0) -= 1;
        }
    }

    evens.values().all(|&v| v == 0) && odds.values().all(|&v| v == 0)
}

fn main() {
    let ret = check_strings("abcdba".to_string(), "cabdab".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(check_strings("abcdba".to_string(), "cabdab".to_string()));
    assert!(!check_strings("abe".to_string(), "bea".to_string()));
}
