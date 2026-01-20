fn vowel_consonant_score(s: String) -> i32 {
    let (v, c) = s.chars().fold((0, 0), |(v, c), ch| match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => (v + 1, c),
        _ => if ch.is_ascii_lowercase() {
            (v, c + 1)
        } else {
            (v, c)
        }
    });

    if c > 0 {
        (v as f64 / c as f64).floor() as i32
    } else {
        0
    }
}

fn main() {
    let ret = vowel_consonant_score("cooear".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(vowel_consonant_score("i3".to_string()), 0);
    assert_eq!(vowel_consonant_score("cooear".to_string()), 2);
    assert_eq!(vowel_consonant_score("axeyizou".to_string()), 1);
    assert_eq!(vowel_consonant_score("au 123".to_string()), 0);
}
