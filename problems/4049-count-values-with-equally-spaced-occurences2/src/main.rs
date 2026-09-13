fn count_special_integers(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for (i, n) in nums.into_iter().enumerate() {
        h.entry(n).or_insert(vec![]).push(i);
    }

    h.into_values().filter(|v| v.len() >= 3).fold(0, |acc, v| {
        let diff = v[1] - v[0];
        for i in 2..v.len() {
            if v[i] - v[i - 1] != diff {
                return acc;
            }
        }

        acc + 1
    })
}

fn main() {
    let ret = count_special_integers(vec![1, 8, 1, 5, 1, 5, 8, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_special_integers(vec![1, 8, 1, 5, 1, 5, 8, 5]), 2);
    assert_eq!(count_special_integers(vec![8, 8, 8, 8]), 1);
    assert_eq!(count_special_integers(vec![8, 6, 6, 8, 8]), 0);
}
