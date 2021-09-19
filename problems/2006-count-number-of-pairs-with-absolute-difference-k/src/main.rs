fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();

    let mut ret = 0;
    for i in 0..len - 1 {
        for j in i + 1..len {
            if (nums[i] - nums[j]).abs() == k {
                ret += 1;
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 2, 1];
    let ret = count_k_difference(nums, 1);
    println!("ret={}", ret);
}

#[test]
fn test_count_k_difference() {
    {
        let nums = vec![1, 2, 2, 1];
        assert_eq!(count_k_difference(nums, 1), 4);
    }
    {
        let nums = vec![1, 3];
        assert_eq!(count_k_difference(nums, 3), 0);
    }
    {
        let nums = vec![3, 2, 1, 5, 4];
        assert_eq!(count_k_difference(nums, 2), 3);
    }
}
