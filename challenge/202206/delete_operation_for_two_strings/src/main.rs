fn min_distance(word1: String, word2: String) -> i32 {
    use std::collections::HashMap;

    fn f(
        word1: &Vec<char>,
        word2: &Vec<char>,
        pos1: i32,
        pos2: i32,
        cache: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        if pos1 < 0 || pos2 < 0 {
            return 0;
        }

        if let Some(&v) = cache.get(&(pos1, pos2)) {
            return v;
        }

        let ret = if word1[pos1 as usize] == word2[pos2 as usize] {
            1 + f(word1, word2, pos1 - 1, pos2 - 1, cache)
        } else {
            let ret1 = f(word1, word2, pos1 - 1, pos2, cache);
            let ret2 = f(word1, word2, pos1, pos2 - 1, cache);
            std::cmp::max(ret1, ret2)
        };

        cache.insert((pos1, pos2), ret);
        ret
    }

    let cs1 = word1.chars().collect();
    let cs2 = word2.chars().collect();
    let len1 = word1.len() as i32;
    let len2 = word2.len() as i32;
    let mut cache = HashMap::new();

    let common_len = f(&cs1, &cs2, len1 - 1, len2 - 1, &mut cache);
    len1 + len2 - (2 * common_len)
}

fn main() {
    let word1 = "leetcode".to_string();
    let word2 = "etco".to_string();
    let ret = min_distance(word1, word2);
    println!("ret={ret}");
}

#[test]
fn test_min_distance() {
    {
        let word1 = "sea".to_string();
        let word2 = "eat".to_string();
        let ret = min_distance(word1, word2);
        assert_eq!(ret, 2);
    }
    {
        let word1 = "leetcode".to_string();
        let word2 = "etco".to_string();
        let ret = min_distance(word1, word2);
        assert_eq!(ret, 4);
    }
}
