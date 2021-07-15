fn custom_sort_string(order: String, str: String) -> String {
    use std::collections::HashMap;

    let mut h: HashMap<u8, usize> = HashMap::new();
    for (i, b) in order.bytes().enumerate() {
        h.insert(b, i);
    }

    let mut j = 26;
    for b in str.bytes() {
        if !h.contains_key(&b) {
            h.insert(b, j);
            j += 1;
        }
    }

    let mut bs: Vec<u8> = str.bytes().collect();
    bs.sort_by(|a, b| {
        let a_val = h.get(a).unwrap();
        let b_val = h.get(b).unwrap();
        a_val.cmp(b_val)
    });

    String::from_utf8(bs).unwrap()
}

fn main() {
    let order = "cba".to_string();
    let str = "abcd".to_string();
    let ret = custom_sort_string(order, str);
    println!("ret={}", ret);
}

#[test]
fn test_custom_sort_string() {
    {
        let order = "cba".to_string();
        let str = "abcd".to_string();
        let ret = custom_sort_string(order, str);
        assert_eq!(ret, "cbad".to_string());
    }
}
