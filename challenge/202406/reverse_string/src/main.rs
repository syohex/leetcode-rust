fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    for i in 0..(len / 2) {
        s.swap(i, len - i - 1);
    }
}

fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    println!("ret={s:?}");
}

#[test]
fn test() {
    {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        let expected = vec!['o', 'l', 'l', 'e', 'h'];
        reverse_string(&mut s);
        assert_eq!(s, expected);
    }
    {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        let expected = vec!['h', 'a', 'n', 'n', 'a', 'H'];
        reverse_string(&mut s);
        assert_eq!(s, expected);
    }
}
