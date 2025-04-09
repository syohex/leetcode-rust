fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    use std::cmp::Ordering;
    use std::collections::HashSet;

    let s: HashSet<_> = nums.into_iter().collect();
    let mut v: Vec<_> = s.into_iter().collect();
    v.sort_by_key(|&n| std::cmp::Reverse(n));

    let last = *v.last().unwrap();
    match last.cmp(&k) {
        Ordering::Less => -1,
        Ordering::Equal => v.len() as i32 - 1,
        Ordering::Greater => v.len() as i32,
    }
}

fn main() {
    let nums = vec![5, 2, 5, 4, 5];
    let k = 2;
    let ret = min_operations(nums, k);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![5, 2, 5, 4, 5];
        let k = 2;
        let ret = min_operations(nums, k);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![2, 1, 2];
        let k = 2;
        let ret = min_operations(nums, k);
        assert_eq!(ret, -1);
    }
    {
        let nums = vec![9, 7, 5, 3];
        let k = 1;
        let ret = min_operations(nums, k);
        assert_eq!(ret, 4);
    }
}
