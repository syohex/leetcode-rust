fn remove_duplicates(s: String) -> String {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        if let Some(prev) = stack.last() {
            if c == *prev {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    stack.into_iter().collect()
}

fn main() {
    let s = "abbaca".to_string();
    let ret = remove_duplicates(s);
    println!("ret={ret}");
}

#[test]
fn test_remove_duplicates() {
    {
        let s = "abbaca".to_string();
        let ret = remove_duplicates(s);
        assert_eq!(ret, "ca");
    }
    {
        let s = "azxxzy".to_string();
        let ret = remove_duplicates(s);
        assert_eq!(ret, "ay");
    }
    {
        let s = "abbb".to_string();
        let ret = remove_duplicates(s);
        assert_eq!(ret, "ab");
    }
    {
        let s = "aaaaaaaaa".to_string();
        let ret = remove_duplicates(s);
        assert_eq!(ret, "a");
    }
}
