fn sum_divisible_by_k(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, n| {
            *acc.entry(n).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .fold(
            0,
            |acc, (n, v)| {
                if v % k == 0 {
                    acc + (n * v)
                } else {
                    acc
                }
            },
        )
}

fn main() {
    let nums = vec![1, 2, 2, 3, 3, 3, 3, 4];
    let ret = sum_divisible_by_k(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 2, 3, 3, 3, 3, 4];
        let ret = sum_divisible_by_k(nums, 2);
        assert_eq!(ret, 16);
    }
    {
        let nums = vec![1, 2, 3, 4, 5];
        let ret = sum_divisible_by_k(nums, 2);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![4, 4, 4, 1, 2, 3];
        let ret = sum_divisible_by_k(nums, 3);
        assert_eq!(ret, 12);
    }
}
