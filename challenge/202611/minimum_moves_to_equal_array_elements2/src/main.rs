fn min_move2(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mid = nums[nums.len() / 2];
    nums.into_iter().fold(0, |acc, n| acc + (n - mid).abs())
}

fn main() {
    let ret = min_move2(vec![1, 10, 2, 9]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_move2(vec![1, 2, 3]), 2);
    assert_eq!(min_move2(vec![1, 10, 2, 9]), 16);
}
