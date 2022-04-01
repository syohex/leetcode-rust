fn reverse_string(s: &mut Vec<char>) {
    let mut left = 0;
    let mut right = s.len() as i32 - 1;

    while left < right {
        s.swap(left as usize, right as usize);

        left += 1;
        right -= 1;
    }
}

fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    println!("ret={:?}", s);
}

#[test]
fn test_reverse_string() {
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
