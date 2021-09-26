fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut ret = -1;
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] < nums[j] {
                ret = std::cmp::max(ret, nums[j] - nums[i]);
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![7, 1, 5, 4];
    println!("ret={}", maximum_difference(nums));
}

#[test]
fn test_maximum_difference() {
    {
        let nums = vec![7, 1, 5, 4];
        assert_eq!(maximum_difference(nums), 4);
    }
    {
        let nums = vec![9, 4, 3, 2];
        assert_eq!(maximum_difference(nums), -1);
    }
    {
        let nums = vec![1, 5, 2, 10];
        assert_eq!(maximum_difference(nums), 9);
    }
}
