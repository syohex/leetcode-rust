fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    fn to_freq(s: &str) -> Vec<i32> {
        s.bytes().fold(vec![0; 26], |mut acc, b| {
            let index = (b - b'a') as usize;
            acc[index] += 1;
            acc
        })
    }

    fn is_subset(a: &[i32], b: &[i32]) -> bool {
        a.iter().zip(b).all(|(i, j)| i >= j)
    }

    let a_freqs: Vec<Vec<i32>> = a.iter().map(|s| to_freq(s)).collect();
    let b_freqs: Vec<Vec<i32>> = b.iter().map(|s| to_freq(s)).collect();

    a_freqs
        .into_iter()
        .zip(a)
        .filter(|(a_freq, _)| b_freqs.iter().all(|b_freq| is_subset(a_freq, b_freq)))
        .map(|(_, word)| word)
        .collect()
}

fn main() {
    let words1 = vec![
        "amazon".to_string(),
        "apple".to_string(),
        "facebook".to_string(),
        "google".to_string(),
        "leetcode".to_string(),
    ];
    let words2 = vec!["e".to_string(), "o".to_string()];
    let ret = word_subsets(words1, words2);
    println!("ret={:?}", ret);
}

#[test]
fn test_word_subsets() {
    {
        let words1 = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let words2 = vec!["e".to_string(), "o".to_string()];
        let expected = vec![
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let ret = word_subsets(words1, words2);
        assert_eq!(ret, expected);
    }
    {
        let words1 = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let words2 = vec!["l".to_string(), "e".to_string()];
        let expected = vec![
            "apple".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let ret = word_subsets(words1, words2);
        assert_eq!(ret, expected);
    }
}
