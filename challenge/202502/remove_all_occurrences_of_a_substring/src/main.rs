fn remove_occurences(s: String, part: String) -> String {
    let p: Vec<_> = part.chars().rev().collect();
    let p_len = part.len();

    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        let len = stack.len();
        if c == p[0]
            && len >= p_len - 1
            && (1..p_len)
                .all(|i| p[i] == stack[len - i])
        {
            for _ in 0..p_len - 1 {
                stack.pop();
            }
        } else {
            stack.push(c);
        }
    }

    stack.into_iter().collect()
}

fn main() {
    let s = "daabcbaabcbc".to_string();
    let part = "abc".to_string();
    let ret = remove_occurences(s, part);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "daabcbaabcbc".to_string();
        let part = "abc".to_string();
        let ret = remove_occurences(s, part);
        assert_eq!(ret, "dab");
    }
    {
        let s = "axxxxyyyyb".to_string();
        let part = "xy".to_string();
        let ret = remove_occurences(s, part);
        assert_eq!(ret, "ab");
    }
}
