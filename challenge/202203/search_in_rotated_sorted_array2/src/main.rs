fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut left = 0 as i32;
    let mut right = (nums.len() - 1) as i32;

    while left <= right {
        let left_val = nums[left as usize];
        let mid = left + ((right - left) / 2);
        let mid_val = nums[mid as usize];
        if mid_val == target {
            return true;
        }

        if left_val < mid_val {
            if mid_val > target && left_val <= target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else if mid_val < left_val {
            if mid_val < target && left_val > target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        } else {
            left += 1;
        }
    }

    false
}

fn main() {
    let nums = vec![2, 5, 6, 0, 0, 1, 2];
    let ret = search(nums, 0);
    println!("ret={ret}");
}

#[test]
fn test_search() {
    {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        assert!(search(nums, 0));
    }
    {
        let nums = vec![2, 5, 6, 0, 0, 1, 2];
        assert!(!search(nums, 3));
    }
}
