fn first_uniq_char(s: String) -> i32 {
    let mut table: Vec<i32> = vec![-1; 26];
    let len = s.len() as i32;
    for (i, b) in s.bytes().enumerate() {
        let index = (b - b'a') as usize;
        table[index] = if table[index] == -1 {
            i as i32
        } else {
            table[index] + len
        }
    }

    let mut ret = len;
    for pos in table {
        if pos != -1 && pos < len && pos < ret {
            ret = pos;
        }
    }

    if ret == len {
        -1
    } else {
        ret
    }
}

fn main() {
    println!(
        "first_uniq_char('loveleetcode')={}",
        first_uniq_char("loveleetcode".to_string())
    );
}

#[test]
fn test_first_unique_char() {
    assert_eq!(first_uniq_char("leetcode".to_string()), 0);
    assert_eq!(first_uniq_char("loveleetcode".to_string()), 2);
    assert_eq!(first_uniq_char("aaaa".to_string()), -1);
}
