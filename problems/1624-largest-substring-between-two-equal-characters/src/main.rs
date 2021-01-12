use std::cmp::max;

fn max_length_between_equal_characters(s: String) -> i32 {
    let bs: Vec<u8> = s.bytes().collect();

    let mut table = vec![s.len(); 26];
    let mut ret: i32 = -1;
    for i in 0..bs.len() {
        let index = (bs[i] - b'a') as usize;
        if table[index] == s.len() {
            table[index] = i;
        } else {
            ret = max(ret, (i - table[index] - 1) as i32);
        }
    }

    ret
}

fn main() {
    println!(
        "max_length_between_equal_characters('cabbac') = {}",
        max_length_between_equal_characters("cabbac".to_string())
    );
}

#[test]
fn test_max_length_between_equal_characters() {
    assert_eq!(max_length_between_equal_characters("aa".to_string()), 0);
    assert_eq!(max_length_between_equal_characters("abca".to_string()), 2);
    assert_eq!(max_length_between_equal_characters("cbzxy".to_string()), -1);
    assert_eq!(max_length_between_equal_characters("cabbac".to_string()), 4);
}
