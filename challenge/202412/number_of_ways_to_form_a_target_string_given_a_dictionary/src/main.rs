fn nums_ways(words: Vec<String>, target: String) -> i32 {
    use std::collections::HashMap;

    fn f(
        i: usize,
        char_table: &Vec<Vec<i64>>,
        j: usize,
        ts: &Vec<u8>,
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        if j >= ts.len() {
            return 1;
        }
        if i >= char_table.len() {
            return 0;
        }

        let key = (i, j);
        if let Some(v) = cache.get(&key) {
            return *v;
        }

        let index = (ts[j] - b'a') as usize;
        let ret1 = char_table[i][index] * f(i + 1, char_table, j + 1, ts, cache);
        let ret2 = f(i + 1, char_table, j, ts, cache);
        let ret = (ret1 + ret2) % 1_000_000_007;

        cache.insert(key, ret);
        ret
    }

    let mut char_table = vec![vec![0; 26]; words[0].len()];

    for s in &words {
        for (i, b) in s.bytes().enumerate() {
            let index = (b - b'a') as usize;
            char_table[i][index] += 1 as i64;
        }
    }

    let mut cache = HashMap::new();
    let ts = target.bytes().collect();
    f(0, &char_table, 0, &ts, &mut cache) as i32
}

fn main() {
    let words = vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()];
    let target = "aba".to_string();
    let ret = nums_ways(words, target);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let words = vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()];
        let target = "aba".to_string();
        let ret = nums_ways(words, target);
        assert_eq!(ret, 6);
    }
    {
        let words = vec!["abba".to_string(), "baab".to_string()];
        let target = "bab".to_string();
        let ret = nums_ways(words, target);
        assert_eq!(ret, 4);
    }
}
