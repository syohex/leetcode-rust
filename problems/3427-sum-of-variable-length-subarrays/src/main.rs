fn subarray_sum(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    for i in 0..nums.len() {
        let start = std::cmp::max(0, i as i32 - nums[i]) as usize;
        for j in start..=i {
            ret += nums[j];
        }
    }
    ret
}

fn main() {
    let nums = vec![2, 3, 1];
    let ret = subarray_sum(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 3, 1];
        let ret = subarray_sum(nums);
        assert_eq!(ret, 11);
    }
    {
        let nums = vec![3, 1, 1, 2];
        let ret = subarray_sum(nums);
        assert_eq!(ret, 13);
    }
}
