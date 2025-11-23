fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut dp = [0, i32::MIN, i32::MIN];

    for n in nums {
        let mut tmp = dp;
        for i in 0..3 {
            let m = n % 3;
            let index = (i + m as usize) % 3;
            tmp[index] = std::cmp::max(tmp[index], dp[i] + n);
        }

        dp = tmp;
    }

    dp[0]
}

fn main() {
    let ret = max_sum_div_three(vec![3, 6, 5, 1, 8]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
    assert_eq!(max_sum_div_three(vec![4]), 0);
    assert_eq!(max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
}
