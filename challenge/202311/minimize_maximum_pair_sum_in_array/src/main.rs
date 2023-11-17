fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = 0;
    let len = nums.len() / 2;
    for i in 0..len {
        ret = std::cmp::max(ret, nums[i] + nums[nums.len() - 1 - i]);
    }

    ret
}

fn main() {
    let nums = vec![3, 5, 4, 2, 4, 6];
    let ret = min_pair_sum(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 5, 2, 3];
        let ret = min_pair_sum(nums);
        assert_eq!(ret, 7);
    }
    {
        let nums = vec![3, 5, 4, 2, 4, 6];
        let ret = min_pair_sum(nums);
        assert_eq!(ret, 8);
    }
}
