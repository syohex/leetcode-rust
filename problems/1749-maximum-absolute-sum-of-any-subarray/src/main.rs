fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut min = 0;
    let mut max_add = 0;
    let mut min_add = 0;
    for &n in nums.iter() {
        max_add = std::cmp::max(max_add + n, n);
        max = std::cmp::max(max, max_add);

        min_add = std::cmp::min(min_add + n, n);
        min = std::cmp::min(min, min_add);
    }

    std::cmp::max(max.abs(), min.abs())
}

fn main() {
    println!("example1={}", max_absolute_sum(vec![1, -3, 2, 3, -4]));
}

#[test]
fn test_max_absolute_sum() {
    assert_eq!(max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
    assert_eq!(max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
}
