fn remove_stars(s: String) -> String {
    let mut ret = vec![];
    for c in s.chars() {
        if c == '*' {
            ret.pop();
        } else {
            ret.push(c);
        }
    }

    ret.into_iter().collect()
}

fn main() {
    let s = "leet**cod*e".to_string();
    let ret = remove_stars(s);
    println!("ret={ret}");
}

#[test]
fn test_remove_stars() {
    {
        let s = "leet**cod*e".to_string();
        let ret = remove_stars(s);
        assert_eq!(ret, "lecoe");
    }
    {
        let s = "erase*****".to_string();
        let ret = remove_stars(s);
        assert_eq!(ret, "");
    }
}
