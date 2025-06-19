fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = 1;
    let mut min = nums[0];

    for n in nums.into_iter().skip(1) {
        if n - min > k {
            min = n;
            ret += 1;
        }
    }

    ret
}

fn main() {
    let nums = vec![3, 6, 1, 2, 5];
    let ret = partition_array(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 6, 1, 2, 5];
        let ret = partition_array(nums, 2);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 2, 3];
        let ret = partition_array(nums, 1);
        assert_eq!(ret, 2);
    }
}
