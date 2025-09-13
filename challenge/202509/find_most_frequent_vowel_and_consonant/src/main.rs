fn max_freq_sum(s: String) -> i32 {
    let mut count = [0; 26];
    for b in s.bytes() {
        count[(b - b'a') as usize] += 1;
    }

    let mut max_vowel = 0;
    let mut max_consonant = 0;
    for (i, c) in count.into_iter().enumerate() {
        match i as u8 + b'a' {
            b'a' | b'e' | b'i' | b'o' | b'u' => max_vowel = std::cmp::max(max_vowel, c),
            _ => max_consonant = std::cmp::max(max_consonant, c),
        }
    }
    max_vowel + max_consonant
}

fn main() {
    let ret = max_freq_sum("successes".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_freq_sum("successes".to_string()), 6);
    assert_eq!(max_freq_sum("aeiaeia".to_string()), 3);
}
