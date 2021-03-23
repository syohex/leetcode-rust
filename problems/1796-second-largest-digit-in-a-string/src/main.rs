fn second_highest(s: String) -> i32 {
    use std::collections::HashSet;
    let h: HashSet<i32> = s
        .bytes()
        .filter(|b| *b >= b'0' && *b <= b'9')
        .map(|b| (b - b'0') as i32)
        .collect();
    if h.len() < 2 {
        -1
    } else {
        let mut v: Vec<&i32> = h.iter().collect();
        v.sort_unstable();
        *v[v.len() - 2]
    }
}

fn main() {
    let ret = second_highest("dfa12321afd".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_second_highest() {
    assert_eq!(second_highest("dfa12321afd".to_string()), 2);
    assert_eq!(second_highest("abc1111".to_string()), -1);
}
