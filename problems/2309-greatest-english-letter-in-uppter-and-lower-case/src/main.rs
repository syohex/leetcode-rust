fn greatest_letter(s: String) -> String {
    let mut lowers = vec![false; 26];
    let mut uppers = vec![false; 26];

    for b in s.bytes() {
        if b >= b'a' && b <= b'z' {
            lowers[(b - b'a') as usize] = true;
        } else {
            uppers[(b - b'A') as usize] = true;
        }
    }

    for i in (0..26usize).rev() {
        if lowers[i] && uppers[i] {
            return ((i as u8 + b'A') as char).to_string();
        }
    }

    "".to_string()
}

fn main() {
    let s = "lEeTcOdE".to_string();
    let ret = greatest_letter(s);
    println!("ret={ret}");
}

#[test]
fn test_greatest_letter() {
    {
        let s = "lEeTcOdE".to_string();
        let ret = greatest_letter(s);
        assert_eq!(&ret, "E");
    }
    {
        let s = "arRAzFif".to_string();
        let ret = greatest_letter(s);
        assert_eq!(&ret, "R");
    }
    {
        let s = "AbCdEfGhIjK".to_string();
        let ret = greatest_letter(s);
        assert_eq!(&ret, "");
    }
}
