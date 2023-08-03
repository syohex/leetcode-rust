fn letter_combinations(digits: String) -> Vec<String> {
    fn f(i: usize, digits: &Vec<u8>, chars: &Vec<&str>, acc: &mut String, ret: &mut Vec<String>) {
        if i == digits.len() {
            ret.push(acc.clone());
            return;
        }

        let index = (digits[i] - b'0' - 2) as usize;
        for c in chars[index].chars() {
            acc.push(c);
            f(i + 1, digits, chars, acc, ret);
            acc.pop();
        }
    }

    let mut ret = vec![];
    if digits.is_empty() {
        return ret;
    }

    let chars = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let digits = digits.bytes().collect();
    let mut acc = String::new();

    f(0, &digits, &chars, &mut acc, &mut ret);
    ret
}

fn main() {
    let digits = "23".to_string();
    let ret = letter_combinations(digits);
    println!("ret={ret:?}");
}

#[test]
fn test_letter_combinations() {
    {
        let digits = "23".to_string();
        let expected = vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string(),
        ];
        let ret = letter_combinations(digits);
        assert_eq!(ret, expected);
    }
    {
        let digits = "".to_string();
        let ret = letter_combinations(digits);
        assert!(ret.is_empty());
    }
    {
        let digits = "2".to_string();
        let expected = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let ret = letter_combinations(digits);
        assert_eq!(ret, expected);
    }
}
