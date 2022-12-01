fn halves_are_alike(s: String) -> bool {
    fn f(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }

    let half = s.len() / 2;
    let cs: Vec<char> = s.chars().collect();

    let mut a_count = 0;
    let mut b_count = 0;
    for i in 0..half {
        if f(cs[i].to_ascii_lowercase()) {
            a_count += 1;
        }
        if f(cs[half + i].to_ascii_lowercase()) {
            b_count += 1;
        }
    }

    a_count == b_count
}

fn main() {
    let s = "book".to_string();
    let ret = halves_are_alike(s);
    println!("ret={ret}");
}

#[test]
fn test_halves_are_alike() {
    {
        let s = "book".to_string();
        assert!(halves_are_alike(s));
    }
    {
        let s = "textbook".to_string();
        assert!(!halves_are_alike(s));
    }
}
