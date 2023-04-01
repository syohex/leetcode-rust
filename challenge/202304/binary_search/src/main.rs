fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0 as i32;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid as i32;
        }

        if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
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
        let ret = search(nums, 9);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let ret = search(nums, 2);
        assert_eq!(ret, -1);
    }
    {
        let nums = vec![5];
        let ret = search(nums, 5);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![5];
        let ret = search(nums, -5);
        assert_eq!(ret, -1);
    }
}
