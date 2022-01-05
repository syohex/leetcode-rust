fn is_palindrome(s: &str) -> bool {
    let cs: Vec<char> = s.chars().collect();
    let len = cs.len();
    let limit = len / 2;
    for i in 0..=limit {
        if cs[i] != cs[len - 1 - i] {
            return false;
        }
    }

    true
}

fn inner(pos: usize, acc: &mut Vec<String>, cs: &Vec<char>, ret: &mut Vec<Vec<String>>) {
    if pos >= cs.len() {
        ret.push(acc.clone());
        return;
    }

    let mut s = String::new();
    for i in pos..cs.len() {
        s.push(cs[i]);
        if is_palindrome(&s) {
            acc.push(s.clone());
            inner(i + 1, acc, cs, ret);
            acc.pop();
        }
    }
}

fn partiton(s: String) -> Vec<Vec<String>> {
    let mut ret = vec![];
    let mut acc = vec![];
    let cs = s.chars().collect();
    inner(0, &mut acc, &cs, &mut ret);
    ret
}

fn main() {
    let ret = partiton("aba".to_string());
    println!("ret={:?}", ret);
}

#[test]
fn test_partition() {
    fn check(got: Vec<Vec<String>>, expected: Vec<Vec<String>>) {
        use std::collections::HashSet;
        let gs: HashSet<Vec<String>> = HashSet::from_iter(got.into_iter());
        let es: HashSet<Vec<String>> = HashSet::from_iter(expected.into_iter());
        assert_eq!(gs, es);
    }

    {
        let expected = vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string()],
            vec!["aa".to_string(), "b".to_string()],
        ];
        let got = partiton("aab".to_string());
        check(got, expected);
    }
    {
        let expected = vec![vec!["a".to_string()]];
        let got = partiton("a".to_string());
        check(got, expected);
    }
    {
        let expected = vec![
            vec!["a".to_string(), "b".to_string(), "a".to_string()],
            vec!["aba".to_string()],
        ];
        let got = partiton("aba".to_string());
        check(got, expected);
    }
}
