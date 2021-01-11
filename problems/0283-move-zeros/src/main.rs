fn move_zeroes(nums: &mut Vec<i32>) {
    let mut pos = 0usize;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[pos] = nums[i];
            pos += 1;
        }
    }

    for _i in 0..(nums.len() - pos) {
        nums[pos] = 0;
        pos += 1;
    }
}

fn main() {
    let mut input = vec![0, 1, 0, 3, 12];
    move_zeroes(&mut input);
    println!("move_zeroes([0, 1, 0, 3, 12]) = {:?}", input);
}

#[test]
fn test_move_zeroes() {
    {
        let mut input = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut input);
        assert_eq!(input, vec![1, 3, 12, 0, 0]);
    }
}
