fn is_middle_element_unique(nums: Vec<i32>) -> bool {
    let mid = nums[nums.len() / 2];
    nums.into_iter().filter(|n| *n == mid).count() == 1
}
fn main() {
    let ret = is_middle_element_unique(vec![1, 2, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(is_middle_element_unique(vec![1, 2, 3]));
    assert!(!is_middle_element_unique(vec![1, 2, 2]));
}
