use std::cmp::max;
use std::collections::HashSet;

fn length_of_longest_substring(s: String) -> i32 {
    let len = s.len();
    let mut i = 0usize;
    let mut j = 0usize;

    let mut hs: HashSet<char> = HashSet::new();
    let mut ret = 0;
    while i < len && j < len {
        let i_ch = s.chars().nth(i).unwrap();
        let j_ch = s.chars().nth(j).unwrap();

        if hs.contains(&j_ch) {
            hs.remove(&i_ch);
            i += 1;
        } else {
            ret = max(ret, (j - i + 1) as i32);
            hs.insert(j_ch);
            j += 1;
        }
    }

    ret
}

fn main() {
    println!(
        "lenght_of_longest_substring('abcabccc') == {}",
        length_of_longest_substring("abcabcbb".to_string())
    );
}

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring("".to_string()), 0);
}
