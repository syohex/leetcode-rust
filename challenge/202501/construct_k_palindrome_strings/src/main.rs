fn can_construct(s: String, k: i32) -> bool {
    let k = k as usize;
    if s.len() < k {
        return false;
    }

    let freq = s.bytes().fold(vec![0; 26], |mut acc, b| {
        let index = (b - b'a') as usize;
        acc[index] += 1;
        acc
    });

    freq.into_iter().filter(|n| n % 2 == 1).count() <= k
}

fn main() {
    let ret = can_construct("annabelle".to_string(), 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(can_construct("annabelle".to_string(), 2));
    assert!(!can_construct("leetcode".to_string(), 3));
    assert!(can_construct("true".to_string(), 4));
    assert!(!can_construct("cr".to_string(), 7));
    assert!(can_construct("abc".to_string(), 3));
}
