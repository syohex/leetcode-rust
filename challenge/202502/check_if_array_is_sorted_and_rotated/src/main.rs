fn check(nums: Vec<i32>) -> bool {
    let last = *nums.last().unwrap();
    nums.into_iter()
        .fold((0, last), |(acc, prev), n| {
            (acc + if prev > n { 1 } else { 0 }, n)
        })
        .0
        <= 1
}
fn main() {
    let nums = vec![3, 4, 5, 1, 2];
    let ret = check(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 4, 5, 1, 2];
        assert!(check(nums));
    }
    {
        let nums = vec![2, 1, 3, 4];
        assert!(!check(nums));
    }
    {
        let nums = vec![1, 2, 3];
        assert!(check(nums));
    }
    {
        let nums = vec![6, 10, 6];
        assert!(check(nums));
    }
    {
        let nums = vec![7, 9, 1, 1, 1, 3];
        assert!(check(nums));
    }
}
