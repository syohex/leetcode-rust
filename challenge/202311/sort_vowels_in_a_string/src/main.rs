fn sort_vowels(s: String) -> String {
    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U' => true,
            _ => false,
        }
    }

    let mut vowels = vec![];
    for c in s.chars() {
        if is_vowel(c) {
            vowels.push(c);
        }
    }

    vowels.sort_unstable();

    let mut ret = String::new();
    let mut i = 0;
    for c in s.chars() {
        if is_vowel(c) {
            ret.push(vowels[i]);
            i += 1;
        } else {
            ret.push(c);
        }
    }

    ret
}

fn main() {
    let s = "lEetcOde".to_string();
    let ret = sort_vowels(s);
    println!("ret={ret}");
}

#[test]
fn test_sort_vowels() {
    {
        let s = "lEetcOde".to_string();
        let expected = "lEOtcede".to_string();
        let ret = sort_vowels(s);
        assert_eq!(ret, expected);
    }
    {
        let s = "lYmpH".to_string();
        let expected = "lYmpH".to_string();
        let ret = sort_vowels(s);
        assert_eq!(ret, expected);
    }
}
