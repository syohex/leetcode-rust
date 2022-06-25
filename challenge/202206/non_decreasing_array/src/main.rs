fn check_possibility(nums: Vec<i32>) -> bool {
    if nums.len() <= 2 {
        return true;
    }

    let mut prev2 = nums[0];
    let mut prev1 = nums[1];

    let mut count = 0;
    if prev2 > prev1 {
        count = 1;
        prev2 = prev1;
    }

    for num in nums.into_iter().skip(2) {
        if prev1 > num {
            count += 1;
            if count == 2 {
                return false;
            }

            if num < prev2 {
                prev2 = prev1;
            } else {
                prev2 = num;
                prev1 = num;
            }
        } else {
            prev2 = prev1;
            prev1 = num;
        }
    }

    true
}

fn main() {
    let nums = vec![4, 2, 3];
    let ret = check_possibility(nums);
    println!("ret={ret}");
}

#[test]
fn test_check_possibility() {
    {
        let nums = vec![4, 2, 3];
        let ret = check_possibility(nums);
        assert!(ret);
    }
    {
        let nums = vec![4, 2, 1];
        let ret = check_possibility(nums);
        assert!(!ret);
    }
    {
        let nums = vec![-1, 4, 2, 3];
        let ret = check_possibility(nums);
        assert!(ret);
    }
    {
        let nums = vec![5, 7, 1, 8];
        let ret = check_possibility(nums);
        assert!(ret);
    }
    {
        let nums = vec![3, 4, 2, 3];
        let ret = check_possibility(nums);
        assert!(!ret);
    }
    {
        let nums = vec![1, 2, 7, 2, 5, 9];
        let ret = check_possibility(nums);
        assert!(ret);
    }
    {
        let nums = vec![1, 2, 5, 4, 3];
        let ret = check_possibility(nums);
        assert!(!ret);
    }
}
