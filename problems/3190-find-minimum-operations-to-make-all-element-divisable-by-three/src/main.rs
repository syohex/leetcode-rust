fn minimum_operations(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, n| match n % 3 {
        0 => acc,
        1 | 2 => acc + 1,
        _ => unreachable!("never reach here"),
    })
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let ret = minimum_operations(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 4];
        let ret = minimum_operations(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![3, 6, 9];
        let ret = minimum_operations(nums);
        assert_eq!(ret, 0);
    }
}
