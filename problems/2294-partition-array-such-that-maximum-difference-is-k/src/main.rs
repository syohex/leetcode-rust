fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut max = *nums.last().unwrap();
    let mut ret = 1;
    for n in nums.into_iter().rev().skip(1) {
        if max - n > k {
            max = n;
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
fn test_partition_array() {
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
    {
        let nums = vec![2, 2, 4, 5];
        let ret = partition_array(nums, 0);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![5];
        let ret = partition_array(nums, 1);
        assert_eq!(ret, 1);
    }
}
