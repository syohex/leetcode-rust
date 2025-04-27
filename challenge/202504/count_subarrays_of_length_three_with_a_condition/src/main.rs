fn count_subarrays(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    for i in 2..nums.len() {
        if (nums[i - 2] + nums[i]) as f64 == nums[i - 1] as f64 / 2.0 {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 1, 4, 1];
    let ret = count_subarrays(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 1, 4, 1];
        let ret = count_subarrays(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![1, 1, 1];
        let ret = count_subarrays(nums);
        assert_eq!(ret, 0);
    }
}
