fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    fn f(i: usize, nums: &[i32], acc: i32) -> i32 {
        if i == nums.len() {
            acc
        } else {
            f(i + 1, nums, acc ^ nums[i]) + f(i + 1, nums, acc)
        }
    }

    f(0, &nums, 0)
}

fn main() {
    let nums = vec![3, 4, 5, 6, 7, 8];
    let ret = subset_xor_sum(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3];
        let ret = subset_xor_sum(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![5, 1, 6];
        let ret = subset_xor_sum(nums);
        assert_eq!(ret, 28);
    }
    {
        let nums = vec![3, 4, 5, 6, 7, 8];
        let ret = subset_xor_sum(nums);
        assert_eq!(ret, 480);
    }
}
