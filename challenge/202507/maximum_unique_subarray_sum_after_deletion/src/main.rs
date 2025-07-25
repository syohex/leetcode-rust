fn max_sum(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let s: HashSet<_> = nums.into_iter().collect();
    if s.iter().all(|&n| n < 0) {
        s.into_iter().max().unwrap()
    } else {
        s.into_iter()
            .fold(0, |acc, n| if n >= 0 { acc + n } else { acc })
    }
}

fn main() {
    let nums = vec![1, 2, -1, -2, 1, 0, -1];
    let ret = max_sum(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![-100, -50];
        let ret = max_sum(nums);
        assert_eq!(ret, -50);
    }
    {
        let nums = vec![-100];
        let ret = max_sum(nums);
        assert_eq!(ret, -100);
    }
    {
        let nums = vec![2, -10, 6];
        let ret = max_sum(nums);
        assert_eq!(ret, 8);
    }
    {
        let nums = vec![1, 1, 0, 1, 1];
        let ret = max_sum(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![1, 2, 3, 4, 5];
        let ret = max_sum(nums);
        assert_eq!(ret, 15);
    }
    {
        let nums = vec![1, 2, -1, -2, 1, 0, -1];
        let ret = max_sum(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![-20, 20];
        let ret = max_sum(nums);
        assert_eq!(ret, 20);
    }
}
