fn min_absolute_difference(nums: Vec<i32>) -> i32 {
    let v: Vec<_> = nums
        .into_iter()
        .enumerate()
        .filter(|(_i, n)| *n != 0)
        .collect();
    let mut ret = i32::MAX;
    for i in 1..v.len() {
        if v[i].1 != v[i - 1].1 {
            ret = std::cmp::min(ret, (v[i].0 - v[i - 1].0) as i32);
        }
    }

    if ret == i32::MAX { -1 } else { ret }
}

fn main() {
    let ret = min_absolute_difference(vec![1, 0, 0, 2, 0, 1]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_absolute_difference(vec![2, 1, 1, 2]), 1);
    assert_eq!(min_absolute_difference(vec![1, 0, 0, 2, 0, 1]), 2);
    assert_eq!(min_absolute_difference(vec![1, 0, 1, 0]), -1);
}
