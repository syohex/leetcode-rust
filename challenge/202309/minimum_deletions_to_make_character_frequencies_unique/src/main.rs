fn min_deletions(s: String) -> i32 {
    use std::collections::{BinaryHeap, HashMap, HashSet};

    let mut freq = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let mut q = BinaryHeap::new();
    for (k, v) in freq.iter() {
        q.push((*v, *k));
    }

    let mut ret = 0;
    let mut checked = HashSet::new();
    while let Some((count, ch)) = q.pop() {
        if count == 0 {
            continue;
        }

        if checked.contains(&count) {
            ret += 1;
            q.push((count - 1, ch));
        } else {
            checked.insert(count);
        }
    }

    ret
}

fn main() {
    let s = "ceabaacb".to_string();
    let ret = min_deletions(s);
    println!("ret={ret}");
}

#[test]
fn test_min_deletions() {
    {
        let s = "aab".to_string();
        let ret = min_deletions(s);
        assert_eq!(ret, 0);
    }
    {
        let s = "aaabbbcc".to_string();
        let ret = min_deletions(s);
        assert_eq!(ret, 2);
    }
    {
        let s = "ceabaacb".to_string();
        let ret = min_deletions(s);
        assert_eq!(ret, 2);
    }
    {
        let s = "bbbbcea".to_string();
        let ret = min_deletions(s);
        assert_eq!(ret, 2);
    }
}
