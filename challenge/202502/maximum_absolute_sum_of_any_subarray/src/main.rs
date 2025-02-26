fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut max_val = 0;

    for &num in &nums {
        sum = std::cmp::max(sum + num, num);
        max_val = std::cmp::max(max_val, sum);
    }

    sum = 0;
    let mut min_val = 0;
    for &num in &nums {
        sum = std::cmp::min(sum + num, num);
        min_val = std::cmp::min(min_val, sum);
    }

    std::cmp::max(max_val.abs(), min_val.abs())
}

fn main() {
    let nums = vec![1, -3, 2, 3, -4];
    let ret = max_absolute_sum(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, -3, 2, 3, -4];
        let ret = max_absolute_sum(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![2, -5, 1, -4, 3, -2];
        let ret = max_absolute_sum(nums);
        assert_eq!(ret, 8);
    }
}
