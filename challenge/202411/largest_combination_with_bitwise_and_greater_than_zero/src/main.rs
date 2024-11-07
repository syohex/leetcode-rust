fn largest_combination(candidates: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn f(
        pos: usize,
        candidates: &Vec<i32>,
        acc: i32,
        len: i32,
        cache: &mut HashMap<(usize, i32, i32), i32>,
    ) -> i32 {
        if pos >= candidates.len() {
            return len;
        }

        let key = (pos, acc, len);
        if let Some(&v) = cache.get(&key) {
            return v;
        }

        let mut ret = f(pos + 1, candidates, acc, len, cache);
        for i in pos..candidates.len() {
            let tmp = acc & candidates[i];
            if tmp != 0 {
                ret = std::cmp::max(ret, f(i + 1, candidates, tmp, len + 1, cache));
            }
        }

        cache.insert(key, ret);
        ret
    }

    let mut cache = HashMap::new();
    f(0, &candidates, 0x7fffffff, 0, &mut cache)
}

fn main() {
    let candidates = vec![16, 17, 71, 62, 12, 24, 14];
    let ret = largest_combination(candidates);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let candidates = vec![16, 17, 71, 62, 12, 24, 14];
        let ret = largest_combination(candidates);
        assert_eq!(ret, 4);
    }
    {
        let candidates = vec![8, 8];
        let ret = largest_combination(candidates);
        assert_eq!(ret, 2);
    }
    {
        let candidates = vec![
            84, 40, 66, 44, 91, 90, 1, 14, 73, 51, 47, 35, 18, 46, 18, 65, 55, 18, 16, 45, 43, 58,
            90, 92, 91, 43, 44, 76, 85, 72, 24, 89, 60, 94, 81, 90, 86, 79, 84, 41, 41, 28, 44,
        ];
        let ret = largest_combination(candidates);
        assert_eq!(ret, 28);
    }
}
