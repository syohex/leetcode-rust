fn max_width_ramp(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut right_largest = vec![nums[len - 1]; nums.len()];

    for i in (0..(len - 1)).rev() {
        right_largest[i] = std::cmp::max(nums[i], right_largest[i + 1]);
    }

    let mut left = 0;
    let mut right = 0;

    let mut ret = 0;
    while right < len {
        while left < right && nums[left] > right_largest[right] {
            left += 1;
        }

        ret = std::cmp::max(ret, (right - left) as i32);
        right += 1;
    }

    ret
}

fn main() {
    let nums = vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1];
    let ret = max_width_ramp(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![6, 0, 8, 2, 1, 5];
        let ret = max_width_ramp(nums);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1];
        let ret = max_width_ramp(nums);
        assert_eq!(ret, 7);
    }
}
