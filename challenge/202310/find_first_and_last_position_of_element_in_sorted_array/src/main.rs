fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn f(nums: &Vec<i32>, target: i32, is_first: bool) -> i32 {
        let last = nums.len() as i32 - 1;
        let mut left = 0;
        let mut right = last;

        while left <= right {
            let mid = left + (right - left) / 2;
            let v = nums[mid as usize];
            if v == target {
                if is_first {
                    if mid == left || nums[mid as usize - 1] != target {
                        return mid;
                    }

                    right = mid - 1;
                } else {
                    if mid == right || nums[mid as usize + 1] != target {
                        return mid;
                    }

                    left = mid + 1;
                }
            } else if v < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }

    let left = f(&nums, target, true);
    if left == -1 {
        return vec![-1, -1];
    }

    vec![left, f(&nums, target, false)]
}

fn main() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let ret = search_range(nums, 8);
    println!("ret={ret:?}");
}

#[test]
fn test_search_range() {
    {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let expected = vec![3, 4];
        let ret = search_range(nums, 8);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let expected = vec![-1, -1];
        let ret = search_range(nums, 6);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![];
        let expected = vec![-1, -1];
        let ret = search_range(nums, 0);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 2, 3];
        let expected = vec![1, 1];
        let ret = search_range(nums, 2);
        assert_eq!(ret, expected);
    }
}
