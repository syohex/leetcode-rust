fn minimum_pushes(word: String) -> i32 {
    let mut freq = [0; 26];
    for b in word.bytes() {
        freq[(b - b'a') as usize] += 1;
    }

    freq.sort_unstable_by_key(|n| std::cmp::Reverse(*n));

    let mut ret = 0;
    for (i, n) in freq.into_iter().enumerate() {
        let base = (i / 8) as i32 + 1;
        ret += base * n;
    }

    ret
}

fn main() {
    let ret = minimum_pushes("xycdefghij".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_pushes("zzzzzabcdefghi".to_string()), 16);
    assert_eq!(minimum_pushes("abcde".to_string()), 5);
    assert_eq!(minimum_pushes("xycdefghij".to_string()), 12);
}
