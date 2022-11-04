fn reverse_vowels(s: String) -> String {
    fn is_vowel(c: char) -> bool {
        let c = c.to_ascii_lowercase();
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }

    let mut v: Vec<char> = s.chars().into_iter().collect();
    let len = v.len() as i32;
    let mut left = 0i32;
    let mut right = len - 1;

    while left < right {
        while left < len && !is_vowel(v[left as usize]) {
            left += 1;
        }

        while right >= 0 && !is_vowel(v[right as usize]) {
            right -= 1;
        }

        if left < right {
            v.swap(left as usize, right as usize);

            left += 1;
            right -= 1;
        }
    }

    v.into_iter().collect()
}

fn main() {
    let s = "leetcode".to_string();
    let ret = reverse_vowels(s);
    println!("ret={ret}");
}

#[test]
fn test_reverse_vowels() {
    {
        let s = "hello".to_string();
        let ret = reverse_vowels(s);
        assert_eq!(ret, "holle");
    }
    {
        let s = "leetcode".to_string();
        let ret = reverse_vowels(s);
        assert_eq!(ret, "leotcede");
    }
    {
        let s = "Ui".to_string();
        let ret = reverse_vowels(s);
        assert_eq!(ret, "iU");
    }
}
