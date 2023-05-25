fn make_smallest_palindrome(s: String) -> String {
    let cs: Vec<char> = s.chars().collect();

    let mut left = 0;
    let mut right = cs.len() - 1;

    let mut ret = vec![cs[cs.len() / 2]; cs.len()];
    while left < right {
        if cs[left] < cs[right] {
            ret[left] = cs[left];
            ret[right] = cs[left];
        } else {
            ret[left] = cs[right];
            ret[right] = cs[right];
        }

        left += 1;
        right -= 1;
    }

    ret.into_iter().collect()
}

fn main() {
    let s = "egcfe".to_string();
    let ret = make_smallest_palindrome(s);
    println!("ret={ret}");
}

#[test]
fn test_make_smallest_palindrome() {
    {
        let s = "egcfe".to_string();
        let ret = make_smallest_palindrome(s);
        assert_eq!(ret, "efcfe");
    }
    {
        let s = "abcd".to_string();
        let ret = make_smallest_palindrome(s);
        assert_eq!(ret, "abba");
    }
    {
        let s = "seven".to_string();
        let ret = make_smallest_palindrome(s);
        assert_eq!(ret, "neven");
    }
    {
        let s = "s".to_string();
        let ret = make_smallest_palindrome(s);
        assert_eq!(ret, "s");
    }
}
