fn permute(ret: &mut Vec<String>, cs: &[char], pos: usize, acc: &mut String) {
    if pos >= cs.len() {
        ret.push(acc.to_string());
    } else {
        if cs[pos].is_ascii_alphabetic() {
            acc.push(cs[pos].to_ascii_lowercase());
            permute(ret, cs, pos + 1, acc);
            acc.pop();

            acc.push(cs[pos].to_ascii_uppercase());
            permute(ret, cs, pos + 1, acc);
            acc.pop();
        } else {
            acc.push(cs[pos]);
            permute(ret, cs, pos + 1, acc);
            acc.pop();
        }
    }
}

fn letter_case_permutation(s: String) -> Vec<String> {
    let mut ret = vec![];
    let cs: Vec<char> = s.chars().collect();
    let mut acc = String::new();
    permute(&mut ret, &cs, 0, &mut acc);
    ret
}

fn main() {
    println!(
        "letter_case_permutation('a1b2')={:?}",
        letter_case_permutation("a1b2".to_string())
    );
}

#[test]
fn test_letter_case_permutation() {
    {
        let input = String::from("a1b2");
        let mut expected: Vec<String> = vec![
            "a1b2".to_string(),
            "a1B2".to_string(),
            "A1b2".to_string(),
            "A1B2".to_string(),
        ];
        let mut ret = letter_case_permutation(input);

        expected.sort_unstable();
        ret.sort_unstable();

        assert_eq!(ret, expected);
    }

    {
        let input = String::from("3z4");
        let mut expected: Vec<String> = vec!["3z4".to_string(), "3Z4".to_string()];
        let mut ret = letter_case_permutation(input);

        expected.sort_unstable();
        ret.sort_unstable();

        assert_eq!(ret, expected);
    }
    {
        let input = String::from("12345");
        let mut expected: Vec<String> = vec!["12345".to_string()];
        let mut ret = letter_case_permutation(input);

        expected.sort_unstable();
        ret.sort_unstable();

        assert_eq!(ret, expected);
    }
    {
        let input = String::from("0");
        let mut expected: Vec<String> = vec!["0".to_string()];
        let mut ret = letter_case_permutation(input);

        expected.sort_unstable();
        ret.sort_unstable();

        assert_eq!(ret, expected);
    }
}
