fn can_make_subsequence(str1: String, str2: String) -> bool {
    let cs1: Vec<u8> = str1.bytes().map(|b| b - b'a').collect();
    let cs2: Vec<u8> = str2.bytes().map(|b| b - b'a').collect();
    let mut i = 0;
    let mut j = 0;
    let (len1, len2) = (str1.len(), str2.len());

    while j < len2 {
        let mut ok = false;
        for k in i..len1 {
            if cs2[j] == cs1[k] || cs2[j] == (cs1[k] + 1) % 26 {
                i = k + 1;
                ok = true;
                break;
            }
        }

        if !ok {
            return false;
        }

        j += 1;
    }

    true
}

fn main() {
    let ret = can_make_subsequence("abc".to_string(), "ad".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(can_make_subsequence("abc".to_string(), "ad".to_string()));
    assert!(can_make_subsequence("zc".to_string(), "ad".to_string()));
    assert!(!can_make_subsequence("ab".to_string(), "d".to_string()));
    assert!(!can_make_subsequence("oh".to_string(), "hu".to_string()));
}
