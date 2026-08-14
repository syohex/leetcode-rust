fn maximum_length_substring(s: String) -> i32 {
    let cs: Vec<_> = s.bytes().collect();
    let mut freq = [0; 26];
    let mut left = 0;
    let mut ret = 0;

    for (i, b) in cs.iter().enumerate() {
        let idx = (*b - b'a') as usize;
        freq[idx] += 1;

        while freq[idx] > 2 {
            freq[(cs[left] - b'a') as usize] -= 1;
            left += 1;
        }

        ret = std::cmp::max(ret, i - left + 1);
    }

    ret as i32
}

fn main() {
    let ret = maximum_length_substring("bcbbbcba".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(maximum_length_substring("bcbbbcba".to_string()), 4);
    assert_eq!(maximum_length_substring("aaaa".to_string()), 2);
}
