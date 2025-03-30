fn partition_labels(s: String) -> Vec<i32> {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        h.insert(c, i);
    }

    let mut ret = vec![];
    let mut start = -1;
    let mut max_pos = 0;
    for (i, c) in s.chars().enumerate() {
        max_pos = std::cmp::max(max_pos, *h.get(&c).unwrap());
        if max_pos == i {
            ret.push(i as i32 - start);
            start = i as i32;
        }
    }

    ret
}

fn main() {
    let s = "ababcbacadefegdehijhklij".to_string();
    let ret = partition_labels(s);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let s = "ababcbacadefegdehijhklij".to_string();
        let expected = vec![9, 7, 8];
        let ret = partition_labels(s);
        assert_eq!(ret, expected);
    }
    {
        let s = "eccbbbbdec".to_string();
        let expected = vec![10];
        let ret = partition_labels(s);
        assert_eq!(ret, expected);
    }
}
