fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let len = nums.len();
    let mut ret = vec![0; len];
    let mask = (1 << maximum_bit) - 1;

    let mut xor = 0;
    for (i, num) in nums.into_iter().enumerate() {
        xor ^= num;
        ret[len - 1 - i] = !xor & mask;
    }

    ret
}

fn main() {
    let nums = vec![0, 1, 2, 2, 5, 7];
    let ret = get_maximum_xor(nums, 3);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![0, 1, 1, 3];
        let expected = vec![0, 3, 2, 3];
        let ret = get_maximum_xor(nums, 2);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![2, 3, 4, 7];
        let expected = vec![5, 2, 6, 5];
        let ret = get_maximum_xor(nums, 3);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![0, 1, 2, 2, 5, 7];
        let expected = vec![4, 3, 6, 4, 6, 7];
        let ret = get_maximum_xor(nums, 3);
        assert_eq!(ret, expected);
    }
}
