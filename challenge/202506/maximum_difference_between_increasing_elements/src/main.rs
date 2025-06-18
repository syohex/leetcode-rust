fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut ret = -1;
    let len = nums.len();
    for i in 0..len {
        for j in (i + 1)..len {
            if nums[i] < nums[j] {
                ret = std::cmp::max(ret, nums[j] - nums[i]);
            }
        }
    }
    ret
}

fn main() {
    let nums = vec![7, 1, 5, 4];
    let ret = maximum_difference(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![7, 1, 5, 4];
        let ret = maximum_difference(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![9, 4, 3, 2];
        let ret = maximum_difference(nums);
        assert_eq!(ret, -1);
    }
    {
        let nums = vec![1, 5, 2, 10];
        let ret = maximum_difference(nums);
        assert_eq!(ret, 9);
    }
}
