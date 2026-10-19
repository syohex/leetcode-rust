fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();
    nums.into_iter()
        .enumerate()
        .filter(|(_, n)| *n == target)
        .map(|(i, _)| i as i32)
        .collect()
}

fn main() {
    let ret = target_indices(vec![1, 2, 5, 2, 3], 2);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
}
