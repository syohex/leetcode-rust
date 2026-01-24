fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    let limit = len / 2;

    let mut ret = 0;
    for i in 0..limit {
        ret = std::cmp::max(ret, nums[i] + nums[len - 1 - i]);
    }

    ret
}

fn main() {
    let ret = min_pair_sum(vec![3, 5, 2, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_pair_sum(vec![3, 5, 2, 3]), 7);
    assert_eq!(min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
}
