fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len() as i32;
    let mut left = 0;
    let mut right = len - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid as usize] == target {
            let mut i = mid - 1;
            while i >= 0 && nums[i as usize] == target {
                i -= 1;
            }

            if i >= 0 && nums[i as usize] == target {
                left = i;
            } else {
                left = i + 1;
            }

            i = mid + 1;
            while i < len && nums[i as usize] == target {
                i += 1;
            }

            if i < len && nums[i as usize] == target {
                right = i;
            } else {
                right = i - 1;
            }

            return vec![left, right];
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    vec![-1, -1]
}

fn main() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let ret = search_range(nums, 8);
    println!("ret={:?}", ret);
}

#[test]
fn test_search_range() {
    {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let ret = search_range(nums, 8);
        assert_eq!(ret, vec![3, 4]);
    }
    {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let ret = search_range(nums, 6);
        assert_eq!(ret, vec![-1, -1]);
    }
    {
        let nums = vec![1, 1, 1, 1, 1];
        let ret = search_range(nums, 1);
        assert_eq!(ret, vec![0, 4]);
    }
    {
        let nums = vec![1, 1, 1, 1, 5];
        let ret = search_range(nums, 1);
        assert_eq!(ret, vec![0, 3]);
    }
    {
        let nums = vec![0, 1, 1, 1, 1];
        let ret = search_range(nums, 1);
        assert_eq!(ret, vec![1, 4]);
    }
    {
        let nums = vec![];
        let ret = search_range(nums, 0);
        assert_eq!(ret, vec![-1, -1]);
    }
}
