fn frequency_sort(s: String) -> String {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for c in s.chars() {
        *h.entry(c).or_insert(0) += 1;
    }

    let mut p = vec![];
    for (k, v) in h.into_iter() {
        p.push((k, v));
    }

    p.sort_unstable_by(|a, b| {
        if a.1 == b.1 {
            return a.0.cmp(&b.0);
        }

        b.1.cmp(&a.1)
    });

    let mut ret = String::new();
    for (k, v) in p {
        for _ in 0..v {
            ret.push(k);
        }
    }
    ret
}

fn main() {
    let ret = frequency_sort("tree".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(frequency_sort("tree".to_string()), "eert");
    assert_eq!(frequency_sort("cccaaa".to_string()), "aaaccc");
    assert_eq!(frequency_sort("Aabb".to_string()), "bbAa");
}
