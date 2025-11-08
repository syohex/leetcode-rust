fn lex_smallest(s: String) -> String {
    let cs: Vec<_> = s.chars().collect();
    let len = s.len();

    let mut ret = cs.clone();
    for i in 2..=len {
        let mut tmp = cs.clone();
        let limit2 = i / 2;
        for j in 0..limit2 {
            tmp.swap(j, i - 1 - j);
        }

        ret = std::cmp::min(ret, tmp);
    }
    for i in 2..=len {
        let mut tmp = cs.clone();
        let limit2 = i / 2;
        for j in 0..limit2 {
            tmp.swap(len - 1 - j, len - i + j);
        }

        ret = std::cmp::min(ret, tmp);
    }

    ret.into_iter().collect()
}

fn main() {
    let ret = lex_smallest("dcab".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(lex_smallest("a".to_string()), "a");
    assert_eq!(lex_smallest("abc".to_string()), "abc");
    assert_eq!(lex_smallest("dcba".to_string()), "abcd");
    assert_eq!(lex_smallest("dcab".to_string()), "acdb");
    assert_eq!(lex_smallest("abba".to_string()), "aabb");
    assert_eq!(lex_smallest("zxy".to_string()), "xzy");
}
