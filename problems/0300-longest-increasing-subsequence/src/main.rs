fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![0; nums.len()];
    dp[0] = 1;

    let mut ret = 1;
    for i in 1..nums.len() {
        let mut n = 1;
        for j in (0..i).rev() {
            if nums[j] < nums[i] {
                n = std::cmp::max(n, dp[j] + 1);
            }
        }

        dp[i] = n;
        ret = std::cmp::max(ret, dp[i]);
    }

    ret
}

fn main() {
    let ret = length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]);
    println!("ret={}", ret);
}

#[test]
fn test_length_of_lis() {
    assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    assert_eq!(length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
    assert_eq!(length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
}
