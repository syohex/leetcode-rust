fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    for i in (0..=(len - 3)).rev() {
        if nums[i] + nums[i + 1] > nums[i + 2] {
            return nums[i] + nums[i + 1] + nums[i + 2];
        }
    }

    0
}

fn main() {
    let nums = vec![2, 1, 2];
    let ret = largest_perimeter(nums);
    println!("ret={ret}");
}

#[test]
fn test_largest_perimeter() {
    {
        let nums = vec![2, 1, 2];
        let ret = largest_perimeter(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![1, 2, 1];
        let ret = largest_perimeter(nums);
        assert_eq!(ret, 0);
    }
}
