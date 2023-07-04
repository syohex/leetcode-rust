fn single_number(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let sum = nums.iter().fold(0i64, |acc, n| acc + *n as i64);
    let uniques: HashSet<i32> = nums.iter().map(|n| *n).collect();
    let unique_sum = uniques.into_iter().fold(0i64, |acc, n| acc + n as i64);

    (((3 * unique_sum) - sum) / 2) as i32
}

fn main() {
    let nums = vec![2, 2, 3, 2];
    let ret = single_number(nums);
    println!("ret={ret}");
}

#[test]
fn test_single_number() {
    {
        let nums = vec![2, 2, 3, 2];
        let ret = single_number(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![0, 1, 0, 1, 0, 1, 99];
        let ret = single_number(nums);
        assert_eq!(ret, 99);
    }
}
