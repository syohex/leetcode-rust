fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = 0;
    let len = nums.len();
    for i in 0..(len - 2) {
        for j in (i + 1)..(len - 1) {
            let mut left = j + 1;
            let mut right = len - 1;

            while left <= right {
                let mid = left + (right - left) / 2;
                if mid == left {
                    break;
                }

                if nums[i] + nums[j] > nums[mid] {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }

            if nums[i] + nums[j] > nums[left] {
                ret = std::cmp::max(ret, nums[i] + nums[j] + nums[left]);
            }
        }
    }
    ret
}

fn main() {
    let nums = vec![1, 2, 1, 10];
    let ret = largest_perimeter(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![3, 2, 3, 4];
        let ret = largest_perimeter(nums);
        assert_eq!(ret, 10);
    }
    {
        let nums = vec![2, 1, 2];
        let ret = largest_perimeter(nums);
        assert_eq!(ret, 5);
    }
    {
        let nums = vec![1, 2, 1, 10];
        let ret = largest_perimeter(nums);
        assert_eq!(ret, 0);
    }
}
