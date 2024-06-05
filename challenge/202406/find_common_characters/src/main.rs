fn common_chars(words: Vec<String>) -> Vec<String> {
    let vs: Vec<Vec<i32>> = words
        .into_iter()
        .map(|s| {
            s.bytes().fold(vec![0; 26], |mut acc, b| {
                acc[(b - b'a') as usize] += 1;
                acc
            })
        })
        .collect();

    let mut table = vs[0].clone();
    for s in vs.iter().skip(1) {
        for i in 0..26 {
            table[i] = std::cmp::min(table[i], s[i]);
        }
    }

    let mut ret = vec![];
    for i in 0..26 {
        for _ in 0..table[i] {
            ret.push(((i as u8 + b'a') as char).to_string());
        }
    }

    ret
}

fn main() {
    let words = vec![
        "bella".to_string(),
        "label".to_string(),
        "roller".to_string(),
    ];
    let ret = common_chars(words);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let words = vec![
            "bella".to_string(),
            "label".to_string(),
            "roller".to_string(),
        ];
        let expected = vec!["e".to_string(), "l".to_string(), "l".to_string()];
        let ret = common_chars(words);
        assert_eq!(ret, expected);
    }
    {
        let words = vec!["cool".to_string(), "lock".to_string(), "cook".to_string()];
        let expected = vec!["c".to_string(), "o".to_string()];
        let ret = common_chars(words);
        assert_eq!(ret, expected);
    }
}
