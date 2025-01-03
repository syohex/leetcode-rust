fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut right: i64 = nums.iter().map(|n| *n as i64).sum();
    let mut left = 0i64;

    let mut ret = 0;
    let len = nums.len();
    for n in nums.into_iter().take(len - 1) {
        let n = n as i64;
        left += n;
        right -= n;

        if left >= right {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let nums = vec![10, 4, -8, 7];
    let ret = ways_to_split_array(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![10, 4, -8, 7];
        let ret = ways_to_split_array(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![2, 3, 1, 0];
        let ret = ways_to_split_array(nums);
        assert_eq!(ret, 2);
    }
}
