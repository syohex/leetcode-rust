fn original_digits(s: String) -> String {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for c in s.chars() {
        *h.entry(c).or_insert(0) += 1;
    }

    let mut v = vec![0; 10];
    if let Some(n) = h.get(&'z') {
        v[0] = *n;
    }
    if let Some(n) = h.get(&'w') {
        v[2] = *n;
    }
    if let Some(n) = h.get(&'u') {
        v[4] = *n;
    }
    if let Some(n) = h.get(&'x') {
        v[6] = *n;
    }
    if let Some(n) = h.get(&'g') {
        v[8] = *n;
    }
    if let Some(n) = h.get(&'h') {
        v[3] = *n - v[8];
    }
    if let Some(n) = h.get(&'f') {
        v[5] = *n - v[4];
    }
    if let Some(n) = h.get(&'s') {
        v[7] = *n - v[6];
    }
    if let Some(n) = h.get(&'i') {
        v[9] = *n - v[5] - v[6] - v[8];
    }
    if let Some(n) = h.get(&'o') {
        v[1] = *n - v[0] - v[2] - v[4];
    }

    let mut ret = vec![];
    for i in 0..=9 {
        for _ in 0..v[i] {
            ret.push(b'0' + i as u8);
        }
    }

    String::from_utf8(ret).unwrap()
}

fn main() {
    let ret = original_digits("owoztneoer".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_original_digits() {
    assert_eq!(original_digits("owoztneoer".to_string()), "012".to_string());
    assert_eq!(original_digits("fviefuro".to_string()), "45".to_string());
}
