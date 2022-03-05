fn delete_and_earn(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;

    let mut h = HashMap::new();
    let mut max_val = std::i32::MIN;
    for num in &nums {
        *h.entry(*num).or_insert(0) += 1;
        max_val = max(max_val, *num);
    }

    let mut dp = vec![0; max_val as usize + 1];
    dp[0] = 0;
    dp[1] = h.get(&1).map_or(0, |x| *x);

    for i in 2..=max_val {
        let val = i * h.get(&i).map_or(0, |x| *x);
        dp[i as usize] = max(dp[i as usize - 1], dp[i as usize - 2] + val);
    }

    dp[max_val as usize]
}

fn main() {
    let nums = vec![2, 2, 3, 3, 3, 4];
    let ret = delete_and_earn(nums);
    println!("ret={ret}");
}

#[test]
fn test_delete_and_earn() {
    {
        let nums = vec![3, 4, 2];
        assert_eq!(delete_and_earn(nums), 6);
    }
    {
        let nums = vec![2, 2, 3, 3, 3, 4];
        assert_eq!(delete_and_earn(nums), 9);
    }
    {
        let nums = vec![1, 3];
        assert_eq!(delete_and_earn(nums), 4);
    }
}
