fn minimum_replacement(nums: Vec<i32>) -> i64 {
    let mut nums = nums;
    let len = nums.len();
    let mut ret = 0i64;
    for i in (0..(len - 1)).rev() {
        let a = nums[i] as i64;
        let b = nums[i + 1] as i64;

        if a <= b {
            continue;
        }

        let count = if a % b == 0 { a / b - 1 } else { a / b };
        ret += count;
        nums[i] = nums[i] / (count + 1) as i32;
    }

    ret
}

fn main() {
    let nums = vec![3, 9, 3];
    let ret = minimum_replacement(nums);
    println!("ret={ret}");
}

#[test]
fn test_minimum_replacement() {
    {
        let nums = vec![3, 9, 3];
        let ret = minimum_replacement(nums);
        assert_eq!(ret, 2i64);
    }
    {
        let nums = vec![1, 2, 3, 4, 5];
        let ret = minimum_replacement(nums);
        assert_eq!(ret, 0i64);
    }
    {
        let nums = vec![12, 9, 7, 6, 17, 19, 21];
        let ret = minimum_replacement(nums);
        assert_eq!(ret, 6i64);
    }
}
