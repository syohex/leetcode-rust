fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;

    let mut dp = vec![HashMap::<i32, i32>::new(); nums.len()];
    let mut ret = 0;

    for i in 0..nums.len() {
        for j in 0..i {
            let diff = nums[i] - nums[j];
            let prev = dp[j].entry(diff).or_insert(1);
            *dp[i].entry(diff).or_default() = *prev + 1;

            ret = max(ret, *dp[i].get(&diff).unwrap());
        }
    }

    ret
}

fn main() {
    let nums = vec![20, 1, 15, 3, 10, 5, 8];
    let ret = longest_arith_seq_length(nums);
    println!("ret={ret}");
}

#[test]
fn test_longest_arith_seq_length() {
    {
        let nums = vec![3, 6, 9, 12];
        let ret = longest_arith_seq_length(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![9, 4, 7, 2, 10];
        let ret = longest_arith_seq_length(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![20, 1, 15, 3, 10, 5, 8];
        let ret = longest_arith_seq_length(nums);
        assert_eq!(ret, 4);
    }
}
