use std::cmp::max;

fn max_product(words: Vec<String>) -> i32 {
    if words.is_empty() {
        return 0;
    }

    let bit_maps: Vec<u32> = words
        .iter()
        .map(|word| {
            word.bytes()
                .fold(0, |bit_map, b| bit_map | (1u32 << (b - b'a')))
        })
        .collect();

    let mut ret = 0usize;
    for i in 0..(words.len() - 1) {
        for j in (i + 1)..words.len() {
            if (bit_maps[i] & bit_maps[j]) == 0 {
                ret = max(ret, words[i].len() * words[j].len());
            }
        }
    }

    ret as i32
}

fn main() {
    let input = vec![
        "abcw".to_string(),
        "baz".to_string(),
        "foo".to_string(),
        "bar".to_string(),
        "xtfn".to_string(),
        "abcdef".to_string(),
    ];

    let ret = max_product(input.clone());
    println!("max_product({:?}) = {}", input, ret);
}

#[test]
fn test_max_product() {
    {
        let input = vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string(),
        ];
        assert_eq!(max_product(input), 16);
    }
    {
        let input = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "cd".to_string(),
            "bcd".to_string(),
            "abcd".to_string(),
        ];
        assert_eq!(max_product(input), 4);
    }
    {
        let input = vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string(),
        ];
        assert_eq!(max_product(input), 0);
    }
    {
        let input = vec![];
        assert_eq!(max_product(input), 0);
    }
}
