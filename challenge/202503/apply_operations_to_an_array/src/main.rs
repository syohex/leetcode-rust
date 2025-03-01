fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let len = nums.len();
    let mut ret = vec![0; len];
    let mut pos = 0;
    for i in 0..len {
        if nums[i] == 0 {
            continue;
        }

        if i < len - 1 && nums[i] == nums[i + 1] {
            ret[pos] = nums[i] * 2;
            nums[i + 1] = 0;
        } else {
            ret[pos] = nums[i];
        }
        pos += 1;
    }
    ret
}

fn main() {
    let nums = vec![1, 2, 2, 1, 1, 0];
    let ret = apply_operations(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![1, 2, 2, 1, 1, 0];
        let expected = vec![1, 4, 2, 0, 0, 0];
        let ret = apply_operations(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![0, 1];
        let expected = vec![1, 0];
        let ret = apply_operations(nums);
        assert_eq!(ret, expected);
    }
}
