fn find_min(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 {
        return nums[0];
    }

    if nums[0] < nums[len - 1] {
        return nums[0];
    }

    let mut left = 0;
    let mut right = len - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] > nums[mid + 1] {
            return nums[mid + 1];
        }

        if nums[mid - 1] > nums[mid] {
            return nums[mid];
        }

        if nums[mid] > nums[0] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    0
}

fn main() {
    let ret = find_min(vec![3, 4, 5, 1, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(find_min(vec![10]), 10);
    assert_eq!(find_min(vec![10, 9]), 9);
    assert_eq!(find_min(vec![10, 11, 9]), 9);
    assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
}
