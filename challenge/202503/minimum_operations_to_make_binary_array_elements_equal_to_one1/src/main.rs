fn min_operations(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len();

    let mut ret = 0;
    for i in 0..(len - 2) {
        if nums[i] == 0 {
            nums[i] = 1;
            nums[i + 1] = if nums[i + 1] == 1 { 0 } else { 1 };
            nums[i + 2] = if nums[i + 2] == 1 { 0 } else { 1 };
            ret += 1;
        }
    }

    if nums.into_iter().sum::<i32>() == len as i32 {
        ret
    } else {
        -1
    }
}

fn main() {
    let nums = vec![0, 1, 1, 1, 0, 0];
    let ret = min_operations(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![0, 1, 1, 1, 0, 0];
        let ret = min_operations(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![0, 1, 1, 1];
        let ret = min_operations(nums);
        assert_eq!(ret, -1);
    }
}
