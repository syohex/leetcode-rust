fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    let k = k as usize;
    let mut ret = i32::MAX;
    for i in 0..=(len - k) {
        ret = std::cmp::min(ret, nums[i + k - 1] - nums[i]);
    }

    ret
}

fn main() {
    let ret = minimum_difference(vec![9, 4, 1, 7], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_difference(vec![90], 1), 0);
    assert_eq!(minimum_difference(vec![9, 4, 1, 7], 2), 2);
}
