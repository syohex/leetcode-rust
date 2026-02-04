fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    for i in (1..len).rev() {
        if nums[i - 1] >= nums[i] {
            return i as i32;
        }
    }

    0
}

fn main() {
    let ret = minimum_prefix_length(vec![1, -1, 2, 3, 3, 4, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_prefix_length(vec![1, -1, 2, 3, 3, 4, 5]), 4);
    assert_eq!(minimum_prefix_length(vec![4, 3, -2, -5]), 3);
    assert_eq!(minimum_prefix_length(vec![1, 2, 3, 4]), 0);
}
