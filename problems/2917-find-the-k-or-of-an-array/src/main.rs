fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
    let mut ret = 0;
    for i in 0..32 {
        let bit = 1 << i;
        let count = nums.iter().filter(|&num| num & bit != 0).count();
        if count >= k as usize {
            ret += bit;
        }
    }

    ret
}

fn main() {
    let nums = vec![7, 12, 9, 8, 9, 15];
    let ret = find_k_or(nums, 4);
    println!("ret={ret}");
}

#[test]
fn test_find_k_or() {
    {
        let nums = vec![7, 12, 9, 8, 9, 15];
        let ret = find_k_or(nums, 4);
        assert_eq!(ret, 9);
    }
    {
        let nums = vec![2, 12, 1, 11, 4, 5];
        let ret = find_k_or(nums, 6);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![10, 8, 5, 9, 11, 6, 8];
        let ret = find_k_or(nums, 1);
        assert_eq!(ret, 15);
    }
}
