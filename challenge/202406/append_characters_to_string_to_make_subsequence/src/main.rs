fn append_characters(s: String, t: String) -> i32 {
    let mut i = 0;
    let mut j = 0;

    let ss: Vec<char> = s.chars().collect();
    let ts: Vec<char> = t.chars().collect();

    while i < s.len() && j < t.len() {
        if ss[i] == ts[j] {
            j += 1;
        }
        i += 1;
    }

    (t.len() - j) as i32
}

fn main() {
    let s = "coaching".to_string();
    let t = "coding".to_string();
    let ret = append_characters(s, t);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        append_characters("coaching".to_string(), "coding".to_string()),
        4
    );
    assert_eq!(append_characters("abcde".to_string(), "a".to_string()), 0);
    assert_eq!(append_characters("z".to_string(), "abcde".to_string()), 5);
    assert_eq!(
        append_characters("lxvqffcj".to_string(), "vjtgatotj".to_string()),
        7
    );
    assert_eq!(append_characters("lbg".to_string(), "g".to_string()), 0);
}
