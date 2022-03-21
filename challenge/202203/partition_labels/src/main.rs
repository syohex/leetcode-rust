fn partition_labels(s: String) -> Vec<i32> {
    use std::cmp::max;
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for (i, c) in s.chars().enumerate() {
        h.insert(c, i as i32);
    }

    let mut ret = vec![];
    let mut max_pos = -1;
    let mut prev = -1;
    for (i, c) in s.chars().enumerate() {
        let i = i as i32;

        max_pos = max(max_pos, *h.get(&c).unwrap());

        if i == max_pos {
            ret.push(i - prev);
            prev = i;
        }
    }

    ret
}

fn main() {
    let s = "ababcbacadefegdehijhklij".to_string();
    let ret = partition_labels(s);
    println!("ret={:?}", ret);
}

#[test]
fn test_partition_labels() {
    {
        let s = "ababcbacadefegdehijhklij".to_string();
        let expected = vec![9, 7, 8];
        assert_eq!(partition_labels(s), expected);
    }
    {
        let s = "eccbbbbdec".to_string();
        let expected = vec![10];
        assert_eq!(partition_labels(s), expected);
    }
}
