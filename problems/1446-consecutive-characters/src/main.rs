use std::cmp::max;

fn max_power(s: String) -> i32 {
    let bs = s.as_bytes();
    let mut prev = bs[0];
    let mut count = 1;
    let mut ret = 0;

    for i in 1..s.len() {
        if prev == bs[i] {
            count += 1;
        } else {
            ret = max(ret, count);
            prev = bs[i];
            count = 1;
        }
    }

    max(ret, count)
}

fn main() {
    println!(
        "max_power('leetcode') = {}",
        max_power("leetcode".to_string())
    );
}

#[test]
fn test_max_power() {
    assert_eq!(max_power("leetcode".to_string()), 2);
    assert_eq!(max_power("abbcccddddeeeeedcba".to_string()), 5);
    assert_eq!(max_power("triplepillooooow".to_string()), 5);
    assert_eq!(max_power("hooraaaaaaaaaaay".to_string()), 11);
    assert_eq!(max_power("tourist".to_string()), 1);
}
