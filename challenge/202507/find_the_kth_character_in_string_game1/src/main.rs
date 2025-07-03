fn kth_character(k: i32) -> char {
    let k = k as usize;
    let mut s = vec![];
    s.push(b'a');

    while s.len() < k {
        let len = s.len();
        for i in 0..len {
            if s[i] == b'z' {
                s.push(b'a');
            } else {
                s.push(s[i] + 1);
            }
        }
    }

    s[k - 1] as char
}

fn main() {
    let ret = kth_character(10);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(kth_character(5), 'b');
    assert_eq!(kth_character(10), 'c');
}
