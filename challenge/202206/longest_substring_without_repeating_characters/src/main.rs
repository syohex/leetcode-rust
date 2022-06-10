fn length_of_longest_substring(s: String) -> i32 {
    let mut v = vec![-1; 256];
    let mut start = 0;

    let mut ret = 0;
    for (i, b) in s.bytes().enumerate() {
        let index = b as usize;
        let i = i as i32;
        if v[index] >= start {
            ret = std::cmp::max(ret, i - start);
            start = v[index] + 1;
        }

        v[index] = i;
    }

    std::cmp::max(ret, s.len() as i32 - start)
}

fn main() {
    let ret = length_of_longest_substring("abcabcbb".to_string());
    println!("ret={ret}");
}

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring("abba".to_string()), 2);
    assert_eq!(length_of_longest_substring("a".to_string()), 1);
    assert_eq!(length_of_longest_substring("ab".to_string()), 2);
    assert_eq!(length_of_longest_substring("aa".to_string()), 1);
    assert_eq!(length_of_longest_substring("aaaabcde".to_string()), 5);
    assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
}
