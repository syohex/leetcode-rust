fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let table = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
        vec!['j', 'k', 'l'],
        vec!['m', 'n', 'o'],
        vec!['p', 'q', 'r', 's'],
        vec!['t', 'u', 'v'],
        vec!['w', 'x', 'y', 'z'],
    ];

    fn f(
        i: usize,
        indexes: &Vec<usize>,
        table: &Vec<Vec<char>>,
        acc: &mut String,
        ret: &mut Vec<String>,
    ) {
        if i >= indexes.len() {
            ret.push(acc.clone());
            return;
        }

        for c in table[indexes[i]].iter() {
            acc.push(*c);
            f(i + 1, indexes, table, acc, ret);
            acc.pop();
        }
    }

    let mut ret = vec![];
    let mut acc = String::new();
    let indexes: Vec<usize> = digits.bytes().map(|b| (b - b'0' - 2) as usize).collect();

    f(0, &indexes, &table, &mut acc, &mut ret);

    ret
}

fn main() {
    let digits = "23".to_string();
    let ret = letter_combinations(digits);
    println!("ret={:?}", ret);
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
