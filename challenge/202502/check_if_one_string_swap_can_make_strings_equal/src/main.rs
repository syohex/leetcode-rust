fn are_almost_equal(s1: String, s2: String) -> bool {
    let v: Vec<_> = s1
        .chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .collect();
    match v.len() {
        0 => true,
        1 => false,
        2 => v[0].0 == v[1].1 && v[0].1 == v[1].0,
        _ => false,
    }
}

fn main() {
    let ret = are_almost_equal("aa".to_string(), "bb".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(are_almost_equal("bank".to_string(), "kanb".to_string()));
    assert!(!are_almost_equal(
        "attack".to_string(),
        "defend".to_string()
    ));
    assert!(are_almost_equal("kelb".to_string(), "kelb".to_string()));
    assert!(!are_almost_equal("aa".to_string(), "bb".to_string()));
    assert!(!are_almost_equal("aa".to_string(), "ac".to_string()));
    assert!(!are_almost_equal("qgqeg".to_string(), "gqgeq".to_string()));
}
