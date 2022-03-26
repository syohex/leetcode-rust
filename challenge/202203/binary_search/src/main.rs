fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0 as i32;
    let mut right = (nums.len() - 1) as i32;

    while left <= right {
        let mid = left + ((right - left) / 2);
        let val = nums[mid as usize];
        if val == target {
            return mid;
        }

        if target < val {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    -1
}

fn main() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let ret = search(nums, 9);
    println!("ret={ret}");
}

#[test]
fn test_search() {
    {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(search(nums, 9), 4);
    }
    {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(search(nums, 2), -1);
    }
    {
        let nums = vec![5];
        assert_eq!(search(nums, 5), 0);
    }
}
