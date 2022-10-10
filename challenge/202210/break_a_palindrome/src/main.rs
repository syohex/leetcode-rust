fn break_palindrome(palindrome: String) -> String {
    if palindrome.len() == 1 {
        return "".to_string();
    }

    let mut cs: Vec<char> = palindrome.chars().collect();
    if palindrome.len() % 2 == 0 {
        for i in 0..cs.len() {
            if cs[i] != 'a' {
                cs[i] = 'a';
                return cs.into_iter().collect();
            }
        }
    } else {
        for i in 0..cs.len() {
            if cs[i] != 'a' {
                if i == palindrome.len() / 2 {
                    for j in (i + 1)..cs.len() {
                        if cs[j] != 'a' {
                            cs[j] = 'a';
                            return cs.into_iter().collect();
                        }
                    }
                } else {
                    cs[i] = 'a';
                    return cs.into_iter().collect();
                }
            }
        }
    }

    let len = cs.len();
    cs[len - 1] = 'b';
    cs.into_iter().collect()
}

fn main() {
    let palindrome = "abccba".to_string();
    let ret = break_palindrome(palindrome);
    println!("ret={ret}");
}

#[test]
fn test_break_palindrome() {
    {
        let palindrome = "abccba".to_string();
        let ret = break_palindrome(palindrome);
        assert_eq!(ret, "aaccba");
    }
    {
        let palindrome = "a".to_string();
        let ret = break_palindrome(palindrome);
        assert_eq!(ret, "");
    }
    {
        let palindrome = "aabaa".to_string();
        let ret = break_palindrome(palindrome);
        assert_eq!(ret, "aabab");
    }
}
