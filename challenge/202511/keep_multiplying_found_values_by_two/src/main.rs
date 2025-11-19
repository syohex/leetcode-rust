fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    use std::collections::HashSet;

    let s: HashSet<_> = nums.into_iter().collect();
    let mut orig = original;

    while let Some(&v) = s.get(&orig) {
        orig = 2 * v;
    }

    orig
}

fn main() {
    let nums = vec![5, 3, 6, 1, 12];
    let ret = find_final_value(nums, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![5, 3, 6, 1, 12];
        let ret = find_final_value(nums, 3);
        assert_eq!(ret, 24);
    }
    {
        let nums = vec![2, 7, 9];
        let ret = find_final_value(nums, 4);
        assert_eq!(ret, 4);
    }
}
