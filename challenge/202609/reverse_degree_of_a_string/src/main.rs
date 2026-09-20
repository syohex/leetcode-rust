fn reverse_degree(s: String) -> i32 {
    s.bytes().enumerate().fold(0, |acc, (i, b)| {
        let score = ((b'z' - b) as i32 + 1) * (i + 1) as i32;
        acc + score
    })
}

fn main() {
    let ret = reverse_degree("abc".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(reverse_degree("abc".to_string()), 148);
    assert_eq!(reverse_degree("zaza".to_string()), 160);
}
