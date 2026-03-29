fn can_be_equal(s1: String, s2: String) -> bool {
    let mut cs: Vec<_> = s1.chars().collect();
    let target: Vec<_> = s2.chars().collect();

    if cs == target {
        return true;
    }

    cs.swap(0, 2);
    if cs == target {
        return true;
    }

    cs.swap(1, 3);
    if cs == target {
        return true;
    }

    cs.swap(0, 2);
    cs == target
}

fn main() {
    let ret = can_be_equal("abcd".to_string(), "cdab".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(can_be_equal("abcd".to_string(), "cdab".to_string()));
    assert!(!can_be_equal("abcd".to_string(), "dacb".to_string()));
}
