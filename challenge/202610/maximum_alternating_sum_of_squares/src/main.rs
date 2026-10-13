fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let mut nums: Vec<_> = nums
        .into_iter()
        .map(|n| {
            let n = n as i64;
            n * n
        })
        .collect();
    nums.sort_unstable();

    let mut ret = 0i64;
    let mut is_right = true;
    let mut right = nums.len() as i32 - 1;
    let mut left = 0i32;

    while left <= right {
        if is_right {
            ret += nums[right as usize];
            right -= 1;
        } else {
            ret -= nums[left as usize];
            left += 1;
        }

        is_right = !is_right;
    }
    ret
}
fn main() {
    let ret = max_alternating_sum(vec![1, 2, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_alternating_sum(vec![1]), 1);
    assert_eq!(max_alternating_sum(vec![1, 2, 3]), 12);
    assert_eq!(max_alternating_sum(vec![1, -1, 2, -2, 3, -3]), 16);
}
