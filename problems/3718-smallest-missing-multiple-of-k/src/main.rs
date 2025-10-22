fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashSet;

    let s: HashSet<_> = nums.into_iter().collect();
    let mut v = 1;
    loop {
        let n = v * k;
        if !s.contains(&n) {
            return n;
        }

        v += 1;
    }
}

fn main() {
    let nums = vec![1, 4, 7, 10, 15];
    let ret = missing_multiple(nums, 5);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![8, 2, 3, 4, 6];
        let ret = missing_multiple(nums, 2);
        assert_eq!(ret, 10);
    }
    {
        let nums = vec![1, 4, 7, 10, 15];
        let ret = missing_multiple(nums, 5);
        assert_eq!(ret, 5);
    }
}
