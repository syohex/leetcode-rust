fn rob(nums: Vec<i32>) -> i32 {
    let mut prev = 0;
    let mut cur = 0;

    for num in nums {
        let tmp = cur;
        cur = std::cmp::max(cur, prev + num);
        prev = tmp;
    }

    cur
}

fn main() {
    let nums = vec![1, 2, 3, 1];
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
