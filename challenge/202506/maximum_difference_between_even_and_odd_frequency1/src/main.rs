fn max_difference(s: String) -> i32 {
    let freq = s.bytes().fold([0; 26], |mut acc, b| {
        acc[(b - b'a') as usize] += 1;
        acc
    });
    let max_odd = *freq.iter().filter(|&&n| n % 2 == 1).max().unwrap();
    let min_even = *freq.iter().filter(|&&n| n != 0 && n % 2 == 0).min().unwrap();

    max_odd - min_even
}

fn main() {
    let ret = max_difference("aaaaabbc".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_difference("aaaaabbc".to_string()), 3);
    assert_eq!(max_difference("abcabcab".to_string()), 1);
}
