pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    let tables = words
        .into_iter()
        .map(|s| {
            s.bytes().fold(vec![0; 26], |mut acc, b| {
                let index = (b - b'a') as usize;
                acc[index] += 1;
                acc
            })
        })
        .collect::<Vec<Vec<i32>>>();

    let mut ret = 0;
    for i in 0..tables.len() {
        for j in (i + 1)..tables.len() {
            if tables[i] == tables[j] {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let words = vec![
        "cd".to_string(),
        "ac".to_string(),
        "dc".to_string(),
        "ca".to_string(),
        "zz".to_string(),
    ];
    let ret = maximum_number_of_string_pairs(words);
    println!("ret={ret}");
}

#[test]
fn test_maximum_number_of_string_pairs() {
    {
        let words = vec![
            "cd".to_string(),
            "ac".to_string(),
            "dc".to_string(),
            "ca".to_string(),
            "zz".to_string(),
        ];
        let ret = maximum_number_of_string_pairs(words);
        assert_eq!(ret, 2);
    }
    {
        let words = vec![
            "ab".to_string(),
            "ba".to_string(),
            "cc".to_string(),
            "ca".to_string(),
            "zz".to_string(),
        ];
        let ret = maximum_number_of_string_pairs(words);
        assert_eq!(ret, 1);
    }
    {
        let words = vec!["aa".to_string(), "ab".to_string()];
        let ret = maximum_number_of_string_pairs(words);
        assert_eq!(ret, 0);
    }
}
