fn frequency_sort(s: String) -> String {
    use std::collections::HashMap;

    let mut freq: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    let mut v: Vec<(char, i32)> = vec![];
    for (&key, &val) in &freq {
        v.push((key, val));
    }

    v.sort_by(|a, b| b.1.cmp(&a.1));

    let mut ret = String::new();
    for elem in &v {
        for _i in 0..elem.1 {
            ret.push(elem.0);
        }
    }

    ret
}

fn main() {
    let ret = frequency_sort("tree".to_string());
    println!("ret={}", ret);
}

#[test]
fn test_frequency_sort() {
    {
        let ret = frequency_sort("tree".to_string());
        assert!(ret == "eetr".to_string() || ret == "eert".to_string());
    }
    {
        let ret = frequency_sort("cccaaa".to_string());
        assert!(ret == "cccaaa".to_string() || ret == "aaaccc".to_string());
    }
    {
        let ret = frequency_sort("Aabb".to_string());
        assert!(ret == "bbAa".to_string() || ret == "bbaA".to_string());
    }
}
