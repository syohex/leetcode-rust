fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut nums = nums;
    for _ in 0..k {
        let mut min = nums[0];
        let mut min_index = 0;
        for (i, &n) in nums.iter().enumerate().skip(1) {
            if n < min {
                min = n;
                min_index = i;
            }
        }

        nums[min_index] *= multiplier;
    }

    nums
}

fn main() {
    let nums = vec![2, 1, 3, 5, 6];
    let ret = get_final_state(nums, 5, 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 1, 3, 5, 6];
        let expected = vec![8, 4, 6, 5, 6];
        let ret = get_final_state(nums, 5, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 2];
        let expected = vec![16, 8];
        let ret = get_final_state(nums, 3, 4);
        assert_eq!(ret, expected);
    }
}
