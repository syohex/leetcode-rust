fn has_same_digits(s: String) -> bool {
    let mut v: Vec<_> = s.bytes().collect();
    while v.len() != 2 {
        let mut tmp = vec![];
        for i in 1..v.len() {
            tmp.push((v[i - 1] + v[i]) % 10);
        }
        v = tmp;
    }

    v[0] == v[1]
}

fn main() {
    let ret = has_same_digits("3902".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(has_same_digits("3902".to_string()));
    assert!(!has_same_digits("34789".to_string()));
}
