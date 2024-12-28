fn minimum_operations(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut s = HashSet::new();
    for (i, n) in nums.into_iter().enumerate().rev() {
        if s.contains(&n) {
            return ((i + 1) as f64 / 3.0).ceil() as i32;
        }
        s.insert(n);
    }

    0
}

fn main() {
    let nums = vec![1, 2, 3, 4, 2, 3, 3, 5, 7];
    let ret = minimum_operations(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 3, 4, 2, 3, 3, 5, 7];
        let ret = minimum_operations(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![4, 5, 6, 4, 4];
        let ret = minimum_operations(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![6, 7, 8, 9];
        let ret = minimum_operations(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![4, 4, 5, 6];
        let ret = minimum_operations(nums);
        assert_eq!(ret, 1);
    }
}
