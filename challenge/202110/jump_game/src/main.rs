fn can_jump(nums: Vec<i32>) -> bool {
    let mut can_jump_pos = (nums.len() - 1) as i32;
    for i in (0..nums.len()).rev() {
        let j = i as i32;
        if j + nums[i] >= can_jump_pos {
            can_jump_pos = j;
        }
    }

    can_jump_pos == 0
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    println!("can_jump({:?}={}", nums.clone(), can_jump(nums));
}

#[test]
fn test_can_jump() {
    assert!(can_jump(vec![2, 3, 1, 1, 4]));
    assert!(!can_jump(vec![3, 2, 1, 0, 4]));
    assert!(can_jump(vec![0]));
}
