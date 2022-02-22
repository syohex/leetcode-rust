fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, n| acc ^ n)
}

fn main() {
    let nums = vec![4, 1, 2, 1, 2];
    let ret = single_number(nums);
    println!("ret={ret}");
}

#[test]
fn test_single_number() {
    {
        let nums = vec![2, 2, 1];
        assert_eq!(single_number(nums), 1);
    }
    {
        let nums = vec![4, 1, 2, 1, 2];
        assert_eq!(single_number(nums), 4);
    }
    {
        let nums = vec![1];
        assert_eq!(single_number(nums), 1);
    }
}
