fn decode_message(key: String, message: String) -> String {
    use std::collections::HashMap;

    let mut h: HashMap<char, char> = HashMap::new();
    let mut i = 0;
    for c in key.chars() {
        if !h.contains_key(&c) && c >= 'a' && c <= 'z' {
            h.insert(c, (b'a' + i) as char);
            i += 1;
        }
    }

    let mut ret = String::new();
    for c in message.chars() {
        if let Some(&d) = h.get(&c) {
            ret.push(d);
        } else {
            ret.push(c);
        }
    }

    ret
}
fn main() {
    let key = "the quick brown fox jumps over the lazy dog".to_string();
    let message = "vkbs bs t suepuv".to_string();
    let ret = decode_message(key, message);
    println!("ret={ret}");
}

#[test]
fn test_decode_message() {
    {
        let key = "the quick brown fox jumps over the lazy dog".to_string();
        let message = "vkbs bs t suepuv".to_string();
        let expected = "this is a secret";
        let ret = decode_message(key, message);
        assert_eq!(ret, expected);
    }
    {
        let key = "eljuxhpwnyrdgtqkviszcfmabo".to_string();
        let message = "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_string();
        let expected = "the five boxing wizards jump quickly";
        let ret = decode_message(key, message);
        assert_eq!(ret, expected);
    }
}
