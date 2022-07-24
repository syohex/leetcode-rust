fn repeated_character(s: String) -> char {
    let mut flag = 0u32;
    for b in s.bytes() {
        let bit = 1 << ((b - b'a') as u32);
        if flag & bit != 0 {
            return b as char;
        }

        flag |= bit;
    }

    '?'
}

fn main() {
    let s = "abccbaacz".to_string();
    let ret = repeated_character(s);
    println!("ret={ret}");
}

#[test]
fn test_repeated_character() {
    {
        let s = "abccbaacz".to_string();
        let ret = repeated_character(s);
        assert_eq!(ret, 'c');
    }
    {
        let s = "abcdd".to_string();
        let ret = repeated_character(s);
        assert_eq!(ret, 'd');
    }
}
