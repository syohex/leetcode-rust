fn max_product(words: Vec<String>) -> i32 {
    let lens: Vec<i32> = words.iter().map(|s| s.len() as i32).collect();
    let tables: Vec<Vec<bool>> = words
        .iter()
        .map(|s| {
            s.bytes().fold(vec![false; 26], |mut acc, b| {
                let index = (b - b'a') as usize;
                acc[index] = true;
                acc
            })
        })
        .collect();

    fn f(a: &[bool], b:&[bool]) -> bool {
        a.into_iter().zip(b).all(|(c, d)| {
            if *c || *d {
                *c != *d
            } else {
                true
            }
        })
    }

    let len = words.len();
    let mut ret = 0;
    for i in 0..len - 1 {
        for j in i + 1..len {
            if f(&tables[i], &tables[j]) {
                ret = std::cmp::max(ret, lens[i] * lens[j]);
            }
        }
    }

    ret
}

fn main() {
    let words = vec![
        "abcw".to_string(),
        "baz".to_string(),
        "foo".to_string(),
        "bar".to_string(),
        "xtfn".to_string(),
        "abcdef".to_string(),
    ];
    let ret = max_product(words);
    println!("ret={ret}");
}

#[test]
fn test_max_product() {
    {
        let words = vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string(),
        ];
        let ret = max_product(words);
        assert_eq!(ret, 16);
    }
    {
        let words = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "cd".to_string(),
            "bcd".to_string(),
            "abcd".to_string(),
        ];
        let ret = max_product(words);
        assert_eq!(ret, 4);
    }
    {
        let words = vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string(),
        ];
        let ret = max_product(words);
        assert_eq!(ret, 0);
    }
}
