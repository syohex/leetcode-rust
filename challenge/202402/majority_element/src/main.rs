fn majority_element(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums[nums.len() / 2]
}

fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    let ret = majority_element(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 2, 3];
        let ret = majority_element(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let ret = majority_element(nums);
        assert_eq!(ret, 2);
    }
}
