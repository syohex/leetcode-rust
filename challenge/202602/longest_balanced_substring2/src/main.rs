fn longest_balanced(s: String) -> i32 {
    use std::collections::HashMap;

    fn one(s: &str) -> i32 {
        let mut ret = 0;
        let mut prev = ' ';
        let mut count = 0;

        for c in s.chars() {
            if c == prev {
                count += 1;
            } else {
                count = 1;
                prev = c;
            }

            ret = std::cmp::max(ret, count);
        }

        ret
    }

    fn two(s: &str, c1: char, c2: char) -> i32 {
        let mut ret = 0;
        let mut h = HashMap::new();
        let mut diff = 0;
        h.insert(0, -1);

        for (i, c) in s.chars().enumerate() {
            if c != c1 && c != c2 {
                diff = 0;
                h.clear();
                h.insert(0, i as i32);
                continue;
            }

            if c == c1 {
                diff += 1;
            } else {
                diff -= 1;
            }

            if let Some(v) = h.get(&diff) {
                ret = std::cmp::max(ret, i as i32 - *v);
            } else {
                h.insert(diff, i as i32);
            }
        }

        ret
    }

    fn three(s: &str) -> i32 {
        let mut ret = 0;
        let mut h = HashMap::new();
        let mut count = [0; 3];
        h.insert((0, 0), -1);

        for (i, c) in s.chars().enumerate() {
            let idx = c as usize - 'a' as usize;
            count[idx] += 1;

            let key = (count[1] - count[0], count[2] - count[1]);
            if let Some(v) = h.get(&key) {
                ret = std::cmp::max(ret, i as i32 - *v);
            } else {
                h.insert(key, i as i32);
            }
        }

        ret
    }

    [
        one(&s),
        two(&s, 'a', 'b'),
        two(&s, 'a', 'c'),
        two(&s, 'b', 'c'),
        three(&s),
    ]
    .into_iter()
    .max()
    .unwrap()
}

fn main() {
    let ret = longest_balanced("abbac".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(longest_balanced("cbbac".to_string()), 3);
    assert_eq!(longest_balanced("bc".to_string()), 2);
    assert_eq!(longest_balanced("abbac".to_string()), 4);
    assert_eq!(longest_balanced("aabcc".to_string()), 3);
    assert_eq!(longest_balanced("aba".to_string()), 2);
}
