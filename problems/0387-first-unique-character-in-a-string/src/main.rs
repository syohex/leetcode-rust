use std::cmp::min;

fn first_uniq_char(s: String) -> i32 {
    let mut table = vec![-1; 26];

    for (i, b) in s.bytes().enumerate() {
        let index = (b - b'a') as usize;
        if table[index] == -1 {
            table[index] = i as i32;
        } else {
            table[index] = (i + s.len()) as i32;
        }
    }

    let mut ret = s.len() as i32;
    for &index in table.iter() {
        if index >= 0 && index < s.len() as i32 {
            ret = min(ret, index);
        }
    }

    if ret == s.len() as i32 {
        -1
    } else {
        ret
    }
}

fn main() {
    println!(
        "first_uniq_char('leetcode') = {}",
        first_uniq_char("leetcode".to_string())
    );
}

#[test]
fn test_first_uniq_char() {
    assert_eq!(first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
    assert_eq!(first_uniq_char("aaaa".to_string()), -1);
}
