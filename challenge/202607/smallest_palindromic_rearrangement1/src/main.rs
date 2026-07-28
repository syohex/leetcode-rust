fn smallest_palindrome(s: String) -> String {
    let len = s.len();
    let mut freq = [0; 26];
    for b in s.bytes() {
        freq[(b - b'a') as usize] += 1;
    }

    let mut ret = vec![' '; len];
    let mut base = 0;
    for (i, count) in freq.into_iter().enumerate() {
        if count == 0 {
            continue;
        }

        let is_odd = count % 2 == 1;
        let c = char::from_u32(i as u32 + b'a' as u32).unwrap();
        for j in 0..(count / 2) {
            ret[base + j] = c;
            ret[len - 1 - j - base] = c;
        }
        base += count / 2;

        if is_odd {
            ret[len / 2] = c;
        }
    }

    ret.into_iter().collect()
}

fn main() {
    let ret = smallest_palindrome("babab".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(smallest_palindrome("jjejj".to_string()), "jjejj");
    assert_eq!(smallest_palindrome("z".to_string()), "z");
    assert_eq!(smallest_palindrome("babab".to_string()), "abbba");
    assert_eq!(smallest_palindrome("daccad".to_string()), "acddca");
}
