fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let len = nums.len();
    for i in (2..len).rev() {
        if nums[i - 2] + nums[i - 1] > nums[i] {
            return nums[i - 2] + nums[i - 1] + nums[i];
        }
    }

    0
}

fn main() {
    let nums = vec![1, 2, 1, 10];
    let ret = largest_perimeter(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 2, 3, 4];
        let ret = largest_perimeter(nums);
        assert_eq!(ret, 10);
    }
    {
        let nums = vec![2, 1, 2];
        let ret = largest_perimeter(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![1, 2, 1, 10];
        let ret = largest_perimeter(nums);
        assert_eq!(ret, 0);
    }
}
