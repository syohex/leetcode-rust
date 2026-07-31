fn minimum_pushes(word: String) -> i32 {
    let mut freq = [0; 26];
    for b in word.bytes() {
        freq[(b - b'a') as usize] += 1;
    }

    freq.sort_unstable_by_key(|n| std::cmp::Reverse(*n));

    freq.into_iter().enumerate().fold(0, |acc, (i, n)| {
        let base = (i as i32 / 8) + 1;
        acc + base * n
    })
}

fn main() {
    let ret = minimum_pushes("xyzxyzxyzxyz".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_pushes("abcde".to_string()), 5);
    assert_eq!(minimum_pushes("xyzxyzxyzxyz".to_string()), 12);
    assert_eq!(minimum_pushes("aabbccddeeffgghhiiiiii".to_string()), 24);
}
