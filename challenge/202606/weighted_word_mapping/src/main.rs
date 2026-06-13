fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
    let mut ret = String::new();
    for word in words {
        let mut sum = 0;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            sum += weights[idx];
        }

        ret.push((b'z' - (sum % 26) as u8) as char);
    }

    ret
}

fn main() {
    let words = vec!["abcd".to_string(), "def".to_string(), "xyz".to_string()];
    let weights = vec![
        5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
    ];
    let ret = map_word_weights(words, weights);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let words = vec!["abcd".to_string(), "def".to_string(), "xyz".to_string()];
        let weights = vec![
            5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
        ];
        let ret = map_word_weights(words, weights);
        assert_eq!(ret, "rij");
    }
    {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let weights = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        let ret = map_word_weights(words, weights);
        assert_eq!(ret, "yyy");
    }
    {
        let words = vec!["abcd".to_string()];
        let weights = vec![
            7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4, 5,
        ];
        let ret = map_word_weights(words, weights);
        assert_eq!(ret, "g");
    }
}
