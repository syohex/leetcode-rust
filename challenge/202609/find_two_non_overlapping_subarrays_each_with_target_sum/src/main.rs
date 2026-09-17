fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    h.insert(0, -1);

    let len = arr.len() as i32;
    let mut min_lens = vec![];
    let mut min_len = len;
    let mut sum = 0;
    let mut ret = len + 1;

    for (i, n) in arr.into_iter().enumerate() {
        sum += n;

        let key = sum - target;
        if let Some(&start) = h.get(&key) {
            let diff = i as i32 - start;

            if start != -1 {
                ret = std::cmp::min(ret, diff + min_lens[start as usize]);
            } else {
                ret = std::cmp::min(ret, diff + len);
            }
            min_len = std::cmp::min(min_len, diff);
        }

        min_lens.push(min_len);
        h.insert(sum, i as i32);
    }

    if ret == len + 1 { -1 } else { ret }
}

fn main() {
    let ret = min_sum_of_lengths(vec![3, 2, 2, 4, 3], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6), -1);
    assert_eq!(min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
    assert_eq!(min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
}
