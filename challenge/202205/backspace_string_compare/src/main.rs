fn backspace_compare(s: String, t: String) -> bool {
    fn f(s: &str) -> String {
        s.chars().fold(String::new(), |mut acc, c| {
            if c == '#' {
                acc.pop();
            } else {
                acc.push(c);
            }
            acc
        })
    }

    f(&s) == f(&t)
}

fn main() {
    let s = "ab#c".to_string();
    let t = "ad#c".to_string();
    let ret = backspace_compare(s, t);
    println!("ret={ret}");
}

#[test]
fn test_backspace_compare() {
    {
        let s = "ab#c".to_string();
        let t = "ad#c".to_string();
        assert!(backspace_compare(s, t));
    }
    {
        let s = "ab##".to_string();
        let t = "c#d#".to_string();
        assert!(backspace_compare(s, t));
    }
    {
        let s = "a#c".to_string();
        let t = "b".to_string();
        assert!(!backspace_compare(s, t));
    }
}
