fn reverse_string(s: &mut Vec<char>) {
    if s.is_empty() {
        return;
    }
    
    let mut i = 0usize;
    let mut j = s.len() - 1;

    while i < j {
        let tmp = s[i];
        s[i] = s[j];
        s[j] = tmp;

        i += 1;
        j -= 1;
    }
}

fn main() {
    let mut v = vec!['h', 'e', 'l', 'l', 'o'];
    println!("orig v = {:?}", v);
    reverse_string(&mut v);
    println!("reverse v = {:?}", v);
}

#[test]
fn test_reverse_string() {
    {
        let mut v = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut v);
        assert_eq!(v, vec!['o', 'l', 'l', 'e', 'h']);
    }
    {
        let mut v = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut v);
        assert_eq!(v, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
