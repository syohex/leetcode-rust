fn custom_sort_string(order: String, s: String) -> String {
    use std::collections::HashMap;

    let mut dict = HashMap::new();
    for (i, c) in order.chars().enumerate() {
        dict.insert(c, i);
    }

    let mut cs: Vec<char> = s.chars().collect();
    cs.sort_by_cached_key(|c| {
        if let Some(v) = dict.get(c) {
            *v
        } else {
            dict.len()
        }
    });

    cs.into_iter().collect()
}

fn main() {
    let order = "bcafg".to_string();
    let s = "abcd".to_string();
    let ret = custom_sort_string(order, s);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let order = "cba".to_string();
        let s = "abcd".to_string();
        let ret = custom_sort_string(order, s);
        assert_eq!(ret, "cbad");
    }
    {
        let order = "bcafg".to_string();
        let s = "abcd".to_string();
        let ret = custom_sort_string(order, s);
        assert_eq!(ret, "bcad");
    }
}
