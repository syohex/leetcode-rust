fn build_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().fold(vec![], |mut acc, &n| {
        acc.push(nums[n as usize]);
        acc
    })
}

fn main() {
    let nums = vec![0, 2, 1, 5, 3, 4];
    let ret = build_array(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![0, 2, 1, 5, 3, 4];
        let expected = vec![0, 1, 2, 4, 5, 3];
        let ret = build_array(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![5, 0, 1, 2, 3, 4];
        let expected = vec![4, 5, 0, 1, 2, 3];
        let ret = build_array(nums);
        assert_eq!(ret, expected);
    }
}
