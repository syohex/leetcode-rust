fn search(nums: Vec<i32>, target: i32) -> i32 {
    fn f(nums: &Vec<i32>, left: i32, right: i32, target: i32) -> i32 {
        let mut left = left;
        let mut right = right;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            }

            if target < nums[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        -1
    }

    let len = nums.len();
    let mut left = 0i32;
    let mut right = len as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] > nums[len - 1] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    let ret = f(&nums, 0, left - 1, target);
    if ret != -1 {
        ret
    } else {
        f(&nums, left, (len - 1) as i32, target)
    }
}

fn main() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let ret = search(nums, 0);
    println!("ret={ret}");
}

#[test]
fn test_search() {
    {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let ret = search(nums, 0);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let ret = search(nums, 3);
        assert_eq!(ret, -1);
    }
    {
        let nums = vec![1];
        let ret = search(nums, 0);
        assert_eq!(ret, -1);
    }
}
