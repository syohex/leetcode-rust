fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut v: Vec<_> = nums.into_iter().enumerate().collect();
    v.sort_unstable_by(|a, b| a.1.cmp(&b.1));

    let mut v: Vec<_> = v.into_iter().rev().take(k as usize).collect();
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    v.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    let nums = vec![2, 1, 3, 3];
    let ret = max_subsequence(nums, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 1, 3, 3];
        let expected = vec![3, 3];
        let ret = max_subsequence(nums, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![-1, -2, 3, 4];
        let expected = vec![-1, 3, 4];
        let ret = max_subsequence(nums, 3);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![3, 4, 3, 3];
        let expected = vec![4, 3];
        let ret = max_subsequence(nums, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![-1, -2, 3, 4];
        let expected = vec![-1, 3, 4];
        let ret = max_subsequence(nums, 3);
        assert_eq!(ret, expected);
    }
}
