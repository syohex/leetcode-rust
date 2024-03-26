fn maximum_length_substring(s: String) -> i32 {
    let cs: Vec<_> = s.bytes().collect();
    let mut left = 0;
    let mut right = 0;

    let mut ret = 0;
    let mut table = vec![0; 26];
    while left < s.len() && right < s.len() {
        let idx = (cs[right] - b'a') as usize;
        table[idx] += 1;
        while table[idx] > 2 && left < s.len() {
            let idx_left = (cs[left] - b'a') as usize;
            table[idx_left] -= 1;
            left += 1;
        }

        ret = std::cmp::max(ret, right - left + 1);
        right += 1;
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
