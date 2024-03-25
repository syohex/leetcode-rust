fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    for i in 0..nums.len() {
        let idx = (nums[i].abs() - 1) as usize;
        nums[idx] *= -1;
    }

    let mut ret = vec![];
    for i in 0..nums.len() {
        let idx = (nums[i].abs() - 1) as usize;
        if nums[idx] > 0 {
            ret.push(nums[i].abs());
            nums[idx] *= -1;
        }
    }

    ret
}
fn main() {
    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    let ret = find_duplicates(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let expected = vec![3, 2];
        let ret = find_duplicates(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 1, 2];
        let expected = vec![1];
        let ret = find_duplicates(nums);
        assert_eq!(ret, expected);
    }
    {
        let ret = find_duplicates(vec![]);
        assert!(ret.is_empty());
    }
}
