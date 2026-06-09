fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let (min, max) = nums
        .into_iter()
        .fold((i64::MAX, i64::MIN), |(min, max), n| {
            let n = n as i64;
            (std::cmp::min(min, n), std::cmp::max(max, n))
        });

    (max - min) * k as i64
}
fn main() {
    let ret = max_total_value(vec![4, 2, 5, 1], 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_total_value(vec![1, 3, 2], 2), 4);
    assert_eq!(max_total_value(vec![4, 2, 5, 1], 3), 12);
}
