fn search(nums: Vec<i32>, target: i32) -> i32 {
    fn f(left: i32, right: i32, target: i32, nums: &[i32]) -> i32 {
        let mut left = left;
        let mut right = right;

        while left <= right {
            let mid = left + (right - left) / 2;
            let v = nums[mid as usize];
            if v == target {
                return mid;
            }

            if target < v {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        -1
    }

    let len = nums.len() as i32;
    let mut left = 0;
    let mut right = len - 1;

    // find minimum number
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] > nums[len as usize - 1] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    let pos = f(0, left - 1, target, &nums);
    if pos != -1 {
        pos
    } else {
        f(left, len - 1, target, &nums)
    }
}

fn main() {
    let ret = search(vec![4, 5, 6, 7, 0, 1, 2], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(search(vec![1], 0), -1);
}
