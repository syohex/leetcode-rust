fn is_valid(word: String) -> bool {
    if word.len() < 3 {
        false
    } else {
        let mut has_vowel = false;
        let mut has_consonant = false;

        for c in word.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    has_vowel = true;
                }
                '@' | '#' | '$' => {
                    return false;
                }
                '1'..='9' => {
                    ()
                }
                _ => {
                    has_consonant = true;
                }
            }
        }

        has_vowel && has_consonant
    }
}

fn main() {
    let ret = is_valid("234aDas".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_valid("234Adas".to_string()));
    assert!(!is_valid("b3".to_string()));
    assert!(!is_valid("a3$e".to_string()));
    assert!(is_valid("AhI".to_string()));
    assert!(!is_valid("UuE6".to_string()));
}
