fn minimum_deletions(word: String, k: i32) -> i32 {
    let freq: Vec<i32> = word
        .bytes()
        .fold(vec![0; 26], |mut acc, b| {
            acc[(b - b'a') as usize] += 1;
            acc
        })
        .into_iter()
        .filter(|&n| n != 0)
        .collect();

    let mut ret = i32::MAX;
    let len = freq.len();
    for i in 0..len {
        let mut deletes = 0;
        for j in 0..len {
            if freq[i] > freq[j] {
                deletes += freq[j];
            } else if freq[j] - freq[i] > k {
                deletes += freq[j] - freq[i] - k;
            }
        }

        ret = std::cmp::min(ret, deletes);
    }
    ret
}

fn main() {
    let ret = minimum_deletions("aabcaba".to_string(), 0);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_deletions("aabcaba".to_string(), 0), 3);
    assert_eq!(minimum_deletions("dabdcbdcdcd".to_string(), 2), 2);
    assert_eq!(minimum_deletions("aaabaaa".to_string(), 2), 1);
}
