fn rob(nums: Vec<i32>) -> i32 {
    let mut prev2 = 0;
    let mut prev1 = 0;

    for num in nums {
        let v = std::cmp::max(prev1, prev2 + num);
        prev2 = prev1;
        prev1 = v;
    }

    prev1
}

fn main() {
    let nums = vec![2, 7, 9, 3, 1];
    let ret = rob(nums);
    println!("ret={ret}");
}

#[test]
fn test_rob() {
    {
        let nums = vec![1, 2, 3, 1];
        let ret = rob(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![2, 7, 9, 3, 1];
        let ret = rob(nums);
        assert_eq!(ret, 12);
    }
}
