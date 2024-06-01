fn score_of_string(s: String) -> i32 {
    let bs: Vec<u8> = s.bytes().collect();
    let mut ret = 0;
    for i in 1..bs.len() {
        ret += (bs[i] as i32 - bs[i - 1] as i32).abs();
    }

    ret
}

fn main() {
    let ret = score_of_string("hello world".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(score_of_string("hello".to_string()), 13);
    assert_eq!(score_of_string("zaz".to_string()), 50);
}
