fn can_jump(nums: Vec<i32>) -> bool {
    let mut can_jump_pos = nums.len() - 1;

    for i in (0..nums.len()).rev() {
        if i + nums[i] as usize >= can_jump_pos {
            can_jump_pos = i;
        }
    }

    can_jump_pos == 0
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    let ret = can_jump(nums);
    println!("ret={ret}");
}

#[test]
fn test_can_jump() {
    {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(can_jump(nums));
    }
    {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!can_jump(nums));
    }
}
