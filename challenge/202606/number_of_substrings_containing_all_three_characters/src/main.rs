fn number_of_substrings(s: String) -> i32 {
    let bs: Vec<_> = s.bytes().collect();
    let len = s.len();
    let mut left = 0;
    let mut right = 0;

    let mut abc = [0; 3];
    let mut ret = 0;
    while right < len {
        abc[(bs[right] - b'a') as usize] += 1;

        while abc.iter().all(|&n| n >= 1) {
            ret += len - right;
            abc[(bs[left] - b'a') as usize] -= 1;
            left += 1;
        }
        right += 1;
    }

    ret as i32
}

fn main() {
    let ret = number_of_substrings("abcabc".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(number_of_substrings("abcabc".to_string()), 10);
    assert_eq!(number_of_substrings("aaacb".to_string()), 3);
    assert_eq!(number_of_substrings("abc".to_string()), 1);
}
