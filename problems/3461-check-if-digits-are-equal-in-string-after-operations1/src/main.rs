fn has_same_digits(s: String) -> bool {
    let mut bs: Vec<_> = s.bytes().collect();
    while bs.len() > 2 {
        let mut tmp = vec![];
        for i in 1..bs.len() {
            let v = (bs[i - 1] + bs[i]) % 10u8;
            tmp.push(v);
        }
        bs = tmp;
    }

    bs[0] == bs[1]
}

fn main() {
    let ret = has_same_digits("34789".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(has_same_digits("3902".to_string()));
    assert!(!has_same_digits("34789".to_string()));
}
