fn number_game(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ret = vec![];
    for i in (0..nums.len()).step_by(2) {
        ret.push(nums[i + 1]);
        ret.push(nums[i]);
    }
    ret
}
fn main() {
    let nums = vec![5, 4, 2, 3];
    let ret = number_game(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![5, 4, 2, 3];
        let expected = vec![3, 2, 5, 4];
        let ret = number_game(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![2, 5];
        let expected = vec![5, 2];
        let ret = number_game(nums);
        assert_eq!(ret, expected);
    }
}
