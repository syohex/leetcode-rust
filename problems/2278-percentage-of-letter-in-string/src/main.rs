fn percentage_letter(s: String, letter: char) -> i32 {
    let count: usize = s.chars().filter(|c| *c == letter).count();
    ((count as f64 / s.len() as f64) * 100.0) as i32
}

fn main() {
    let ret = percentage_letter("foobar".to_string(), 'o');
    println!("ret={ret}");
}

#[test]
fn test_percentage_letter() {
    assert_eq!(percentage_letter("foobar".to_string(), 'o'), 33);
    assert_eq!(percentage_letter("foooor".to_string(), 'o'), 66);
    assert_eq!(percentage_letter("jjjj".to_string(), 'k'), 0);
}
