fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let len = nums.len();

    for i in 0..len {
        for j in i..len {
            if (nums[i] - nums[j]).abs() <= std::cmp::min(nums[i], nums[j]) {
                ret = std::cmp::max(ret, nums[i] ^ nums[j]);
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let ret = maximum_strong_pair_xor(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 4, 5];
        let ret = maximum_strong_pair_xor(nums);
        assert_eq!(ret, 7);
    }
    {
        let nums = vec![10, 100];
        let ret = maximum_strong_pair_xor(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![5, 6, 25, 30];
        let ret = maximum_strong_pair_xor(nums);
        assert_eq!(ret, 7);
    }
}
