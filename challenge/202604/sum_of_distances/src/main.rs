fn distance(nums: Vec<i32>) -> Vec<i64> {
    use std::collections::HashMap;

    let mut ret = vec![0; nums.len()];
    let mut groups = HashMap::new();
    for (i, n) in nums.into_iter().enumerate() {
        groups.entry(n).or_insert(vec![]).push(i as i64);
    }

    for indexes in groups.into_values() {
        let len = indexes.len();
        let mut prefix = vec![0; len + 1];
        for (i, idx) in indexes.iter().enumerate() {
            prefix[i + 1] = prefix[i] + *idx;
        }

        for (i, idx) in indexes.into_iter().enumerate() {
            let left = i as i64 * idx - prefix[i];
            let right = prefix[len] - prefix[i] - (len - i) as i64 * idx;
            ret[idx as usize] = left + right;
        }
    }

    ret
}

fn main() {
    let ret = distance(vec![1, 3, 1, 1, 2]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(distance(vec![1, 3, 1, 1, 2]), vec![5, 0, 3, 4, 0]);
    assert_eq!(distance(vec![0, 5, 3]), vec![0, 0, 0]);
}
