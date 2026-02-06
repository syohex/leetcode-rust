fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as i64;
    let mut nums : Vec<i64> = nums.into_iter().map(|n| n as i64).collect();
    nums.sort_unstable();

    let len = nums.len();
    let mut left = 0;
    let mut ret = i32::MAX;
    for right in 0..len {
        while left < right && nums[left] * k < nums[right] {
            left += 1
        }

        let window_size = right - left + 1;
        ret = std::cmp::min(ret, (len - window_size) as i32);
    }

    ret
}

fn main() {
    let ret = min_removal(vec![2, 1, 5], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_removal(vec![1, 34, 23], 2), 1);
    assert_eq!(min_removal(vec![2, 1, 5], 2), 1);
    assert_eq!(min_removal(vec![1, 6, 2, 9], 3), 2);
    assert_eq!(min_removal(vec![4, 6], 2), 0);
}
