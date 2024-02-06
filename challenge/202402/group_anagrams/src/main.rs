fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut h: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
    for s in strs {
        let mut v = vec![0; 26];
        for b in s.bytes() {
            let index = (b - b'a') as usize;
            v[index] += 1;
        }

        h.entry(v).or_insert(vec![]).push(s);
    }

    h.values().cloned().collect()
}

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let ret = group_anagrams(strs);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let ret = group_anagrams(strs);
        assert_eq!(ret.len(), 3);
    }
    {
        let strs = vec!["".to_string()];
        let ret = group_anagrams(strs);
        assert_eq!(ret.len(), 1);
    }
    {
        let strs = vec!["a".to_string()];
        let ret = group_anagrams(strs);
        assert_eq!(ret.len(), 1);
    }
}
