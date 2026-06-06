fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut left_sum = vec![0; len];
    let mut right_sum = vec![0; len];

    let mut sum = 0;
    for i in 0..len {
        left_sum[i] = sum;
        sum += nums[i];
    }

    sum = 0;
    for i in (0..len).rev() {
        right_sum[i] = sum;
        sum += nums[i];
    }

    left_sum
        .into_iter()
        .zip(right_sum)
        .map(|(l, r)| (l - r).abs())
        .collect()
}

fn main() {
    let ret = left_right_difference(vec![10, 4, 8, 3]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(left_right_difference(vec![10, 4, 8, 3]), [15, 1, 11, 22]);
    assert_eq!(left_right_difference(vec![1]), [0]);
}
