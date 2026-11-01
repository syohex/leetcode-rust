fn min_moves(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut moves = 0;
    for i in 1..nums.len() {
        let diff = (moves + nums[i]) - nums[i - 1];
        nums[i] += moves;
        moves += diff;
    }

    moves
}

fn main() {
    let ret = min_moves(vec![1, 2, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_moves(vec![1, 2, 3]), 3);
    assert_eq!(min_moves(vec![1, 1, 1]), 0);
}
