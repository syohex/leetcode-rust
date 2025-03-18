fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut ret = 1;

    let mut bits = 0;
    for (i, &num) in nums.iter().enumerate() {
        while (bits & num) != 0 {
            bits ^= nums[left];
            left += 1;
        }

        bits |= num;
        ret = std::cmp::max(ret, i - left + 1);
    }

    ret as i32
}

fn main() {
    let nums = vec![1, 3, 8, 48, 10];
    let ret = longest_nice_subarray(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 3, 8, 48, 10];
        let ret = longest_nice_subarray(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![3, 1, 5, 11, 13];
        let ret = longest_nice_subarray(nums);
        assert_eq!(ret, 1);
    }
}
