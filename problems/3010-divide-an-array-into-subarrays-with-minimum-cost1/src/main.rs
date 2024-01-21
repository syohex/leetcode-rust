fn minimum_cost(nums: Vec<i32>) -> i32 {
    let mut v = vec![];

    for num in nums.iter().skip(1) {
        v.push(*num);
    }

    v.sort_unstable();

    nums[0] + v[0] + v[1]
}
fn main() {
    let nums = vec![1, 2, 3, 12];
    let ret = minimum_cost(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 12];
        let ret = minimum_cost(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![5, 4, 3];
        let ret = minimum_cost(nums);
        assert_eq!(ret, 12);
    }
    {
        let nums = vec![10, 3, 1, 1];
        let ret = minimum_cost(nums);
        assert_eq!(ret, 12);
    }
}
