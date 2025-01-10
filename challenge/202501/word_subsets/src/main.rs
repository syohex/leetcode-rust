fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    fn to_table(s: &str) -> Vec<i32> {
        let mut ret = vec![0; 26];
        for b in s.bytes() {
            let index = (b - b'a') as usize;
            ret[index] += 1;
        }
        ret
    }

    fn is_subset(v1: &[i32], v2: &[i32]) -> bool {
        for i in 0..26 {
            if v1[i] < v2[i] {
                return false;
            }
        }
        true
    }

    let mut table2 = vec![0; 26];
    for word2 in words2 {
        let t = to_table(&word2);
        for i in 0..26 {
            table2[i] = std::cmp::max(table2[i], t[i]);
        }
    }

    words1
        .into_iter()
        .filter(|w1| {
            let t1 = to_table(w1);
            is_subset(&t1, &table2)
        })
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
    println!("ret={ret:?}");
}

#[test]
fn test() {
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
