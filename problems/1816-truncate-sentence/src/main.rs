fn truncate_sentence(s: String, k: i32) -> String {
    let bs: Vec<u8> = s.bytes().collect();
    let mut count = 0;
    for i in 0..bs.len() {
        if bs[i] == b' ' {
            count += 1;
            if count == k {
                return String::from_utf8_lossy(&bs[..i]).to_string();
            }
        }
    }

    s
}

fn main() {
    let ret = truncate_sentence("Hello how are you Contestant".to_string(), 4);
    println!("ret={}", ret);
}

#[test]
fn test_truncate_sentence() {
    assert_eq!(
        truncate_sentence("Hello how are you Contestant".to_string(), 4),
        "Hello how are you".to_string()
    );
    assert_eq!(
        truncate_sentence("What is the solution to this problem".to_string(), 4),
        "What is the solution".to_string()
    );
    assert_eq!(
        truncate_sentence("chopper is not a tanuki".to_string(), 5),
        "chopper is not a tanuki".to_string()
    );
}
