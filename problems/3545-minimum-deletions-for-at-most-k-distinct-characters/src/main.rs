fn min_deletion(s: String, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut v : Vec<_> = s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    }).into_values().collect();

    v.sort_unstable();

    let k = k as usize;
    if v.len() <= k {
        0
    } else {
        let n = v.len() - k;
        v.into_iter().take(n).sum()
    }
}

fn main() {
    let ret = min_deletion("abc".to_string(), 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_deletion("abc".to_string(), 2), 1);
    assert_eq!(min_deletion("aabb".to_string(), 2), 0);
}
