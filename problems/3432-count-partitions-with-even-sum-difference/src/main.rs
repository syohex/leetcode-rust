fn count_partitions(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut ret = 0;
    let mut left = 0;
    let mut right: i32 = nums.iter().sum();

    for n in nums.into_iter().take(len - 1) {
        left += n;
        right -= n;

        if (left - right) % 2 == 0 {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let nums = vec![10, 10, 3, 7, 6];
    let ret = count_partitions(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![10, 10, 3, 7, 6];
        let ret = count_partitions(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![1, 2, 2];
        let ret = count_partitions(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![2, 4, 6, 8];
        let ret = count_partitions(nums);
        assert_eq!(ret, 3);
    }
}
