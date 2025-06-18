fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = vec![];
    for i in (0..nums.len()).step_by(3) {
        if nums[i + 2] - nums[i] > k {
            return vec![];
        }

        ret.push(vec![nums[i], nums[i + 1], nums[i + 2]]);
    }

    ret
}

fn main() {
    let nums = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
    let ret = divide_array(nums, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
        let expected = vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]];
        let ret = divide_array(nums, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![2, 4, 2, 2, 5, 2];
        let ret = divide_array(nums, 2);
        assert!(ret.is_empty());
    }
}
