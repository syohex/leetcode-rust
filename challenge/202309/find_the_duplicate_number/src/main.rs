fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut left = 1i32;
    let mut right = nums.len() as i32;

    let mut ret = 0;
    while left <= right {
        let mid = left + (right - left) / 2;

        let count = nums.iter().filter(|&&n| n <= mid).count();
        if count > mid as usize {
            ret = mid;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 3, 4, 2, 2];
    let ret = find_duplicate(nums);
    println!("ret={ret}");
}

#[test]
fn test_find_duplicate() {
    {
        let nums = vec![1, 3, 4, 2, 2];
        let ret = find_duplicate(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![3, 1, 3, 4, 2];
        let ret = find_duplicate(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 1];
        let ret = find_duplicate(nums);
        assert_eq!(ret, 1);
    }
}
