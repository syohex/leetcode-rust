fn minimum_swaps(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut nums = nums;

    let mut ret = 0;
    while left < right {
        if nums[right] != 0 {
            while left < right && nums[left] != 0 {
                left += 1;
            }
            if left >= right {
                break;
            }
            nums.swap(left, right);
            left += 1;
            ret += 1;
        }
        right -= 1;
    }

    ret
}

fn main() {
    let ret = minimum_swaps(vec![0, 1, 0, 3, 12]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_swaps(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(minimum_swaps(vec![0, 0, 0, 0, 0]), 0);
    assert_eq!(minimum_swaps(vec![0, 1, 0, 3, 12]), 2);
    assert_eq!(minimum_swaps(vec![0, 1, 0, 2]), 1);
}
