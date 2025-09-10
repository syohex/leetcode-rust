fn minimum_teaching(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;

    let mut no_common_languages = HashSet::new();
    for f in &friendships {
        let mut known_languages = HashSet::new();
        for lang in &languages[f[0] as usize - 1] {
            known_languages.insert(lang);
        }

        if languages[f[1] as usize - 1]
            .iter()
            .all(|lang| !known_languages.contains(lang))
        {
            no_common_languages.insert(f[0]);
            no_common_languages.insert(f[1]);
        }
    }

    let mut language_count = vec![0; n as usize];
    let mut max_language = 0;
    for &id in &no_common_languages {
        for &lang in &languages[id as usize - 1] {
            let idx = lang as usize - 1;
            language_count[idx] += 1;
            max_language = std::cmp::max(max_language, language_count[idx]);
        }
    }

    (no_common_languages.len() - max_language) as i32
}

fn main() {
    let n = 2;
    let languages = vec![vec![1], vec![2], vec![1, 2]];
    let friendships = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    let ret = minimum_teaching(n, languages, friendships);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let n = 2;
        let languages = vec![vec![1], vec![2], vec![1, 2]];
        let friendships = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        let ret = minimum_teaching(n, languages, friendships);
        assert_eq!(ret, 1);
    }
    {
        let n = 3;
        let languages = vec![vec![2], vec![1, 3], vec![1, 2], vec![3]];
        let friendships = vec![vec![1, 4], vec![1, 2], vec![3, 4], vec![2, 3]];
        let ret = minimum_teaching(n, languages, friendships);
        assert_eq!(ret, 2);
    }
}
