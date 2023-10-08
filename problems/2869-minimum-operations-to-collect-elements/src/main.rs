fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let mask = (1u64 << k as u64) - 1;

    let mut ret = 0;
    let mut bits = 0u64;
    while let Some(n) = nums.pop() {
        ret += 1;
        bits |= 1u64 << (n as u64 - 1);
        if (bits & mask) == mask {
            break;
        }
    }

    ret
}

fn main() {
    let nums = vec![3, 1, 5, 4, 2];
    let ret = min_operations(nums, 2);
    println!("ret={ret}");
}

#[test]
fn test_min_operations() {
    {
        let nums = vec![3, 1, 5, 4, 2];
        let ret = min_operations(nums, 2);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![3, 1, 5, 4, 2];
        let ret = min_operations(nums, 5);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![3, 2, 5, 3, 1];
        let ret = min_operations(nums, 3);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![3,28,33,26,34,20,27,5,21,23,4,21,37,35,32,15,14,1,7,2,9,6,38,17,30,18,16,13,24,29,12,14,8,36,11,31,25,22,10,19];
        let ret = min_operations(nums, 38);
        assert_eq!(ret, 40);
    }
}
