fn count_characters(words: Vec<String>, chars: String) -> i32 {
    fn to_count(s: &str) -> Vec<usize> {
        let mut v = vec![0; 26];
        for b in s.bytes() {
            v[(b - b'a') as usize] += 1;
        }
        v
    }

    let table = to_count(&chars);
    words.into_iter().fold(0, |acc, word| {
        let t = to_count(&word);
        for i in 0..26 {
            if table[i] < t[i] {
                return acc;
            }
        }

        acc + word.len() as i32
    })
}

fn main() {
    let words = vec![
        "hello".to_string(),
        "world".to_string(),
        "leetcode".to_string(),
    ];
    let chars = "welldonehoneyr".to_string();
    let ret = count_characters(words, chars);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let words = vec![
            "cat".to_string(),
            "bt".to_string(),
            "hat".to_string(),
            "tree".to_string(),
        ];
        let chars = "atach".to_string();
        let ret = count_characters(words, chars);
        assert_eq!(ret, 6);
    }
    {
        let words = vec![
            "hello".to_string(),
            "world".to_string(),
            "leetcode".to_string(),
        ];
        let chars = "welldonehoneyr".to_string();
        let ret = count_characters(words, chars);
        assert_eq!(ret, 10);
    }
}
