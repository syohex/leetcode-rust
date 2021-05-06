fn move_zeros(nums: &mut Vec<i32>) {
    let mut pos = 0usize;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[pos] = nums[i];
            pos += 1
        }
    }

    for i in pos..nums.len() {
        nums[i] = 0;
    }
}

fn main() {
    let mut nums = vec![0, 1, 0, 3, 12];
    move_zeros(&mut nums);
    println!("ret={:?}", nums);
}

#[test]
fn test_move_zeros() {
    {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeros(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
    {
        let mut nums = vec![0];
        move_zeros(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}
