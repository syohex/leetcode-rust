fn compressed_string(word: String) -> String {
    let cs: Vec<char> = word.chars().collect();
    let mut count = 1;
    let mut prev = cs[0];

    let mut ret = String::new();
    for i in 1..cs.len() {
        if count == 9 {
            ret.push((count as u8 + b'0') as char);
            ret.push(prev);
            count = 1;
            prev = cs[i];
        } else if cs[i] == prev {
            count += 1;
        } else {
            ret.push_str(&count.to_string());
            ret.push(prev);
            prev = cs[i];
            count = 1;
        }
    }

    ret.push((count as u8 + b'0') as char);
    ret.push(prev);

    ret
}

fn main() {
    let ret = compressed_string("aaaaaaaaaaaaaabb".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(compressed_string("abcde".to_string()), "1a1b1c1d1e");
    assert_eq!(compressed_string("aaaaaaaaaaaaaabb".to_string()), "9a5a2b");
    assert_eq!(compressed_string("aaaaaaaaa".to_string()), "9a");
    assert_eq!(compressed_string("aaaaaaaaaa".to_string()), "9a1a");
    assert_eq!(compressed_string("aaaaaaaaay".to_string()), "9a1y");
}
