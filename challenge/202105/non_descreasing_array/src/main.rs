fn check_possiblity(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    let mut diffs = 0;
    for i in 1..nums.len() {
        if nums[i - 1] > nums[i] {
            diffs += 1;
            if diffs >= 2 {
                return false;
            }

            if i < 2 || nums[i - 2] <= nums[i] {
                nums[i - 1] = nums[i];
            } else {
                nums[i] = nums[i - 1];
            }
        }
    }

    true
}

fn main() {
    let ret = check_possiblity(vec![4, 2, 3]);
    println!("ret={}", ret);
}

#[test]
fn test_check_possibility() {
    assert!(check_possiblity(vec![4, 2, 3]));
    assert!(!check_possiblity(vec![4, 2, 1]));
    assert!(!check_possiblity(vec![3, 4, 2, 3]));
    assert!(check_possiblity(vec![5, 7, 1, 8]));
    assert!(check_possiblity(vec![-1, 4, 2, 3]));
}
