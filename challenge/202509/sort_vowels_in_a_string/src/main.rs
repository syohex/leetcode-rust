fn sort_vowels(s: String) -> String {
    let mut vowels = vec![];
    for c in s.chars() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels.push(c),
            _ => (),
        }
    }

    vowels.sort_unstable();

    let mut ret = String::new();
    let mut i = 0;
    for c in s.chars() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                ret.push(vowels[i]);
                i += 1;
            }
            _ => ret.push(c),
        }
    }

    ret
}

fn main() {
    let ret = sort_vowels("lEetcOde".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sort_vowels("lEetcOde".to_string()), "lEOtcede");
    assert_eq!(sort_vowels("lYmpH".to_string()), "lYmpH");
}
