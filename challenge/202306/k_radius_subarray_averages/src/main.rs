fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let len = 2 * k + 1;
    if len > nums.len() {
        return vec![-1; nums.len()];
    }

    let mut ret = vec![-1; nums.len()];
    let mut sum = 0i64;
    for i in 0..len {
        sum += nums[i] as i64;
    }

    let mut mid = len - k - 1;
    ret[mid] = (sum / len as i64) as i32;
    mid += 1;

    for i in len..nums.len() {
        sum = sum + nums[i] as i64 - nums[i - len] as i64;
        ret[mid] = (sum / len as i64) as i32;
        mid += 1;
    }

    ret
}

fn main() {
    let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
    let ret = get_averages(nums, 3);
    println!("ret={ret:?}");
}

#[test]
fn test_get_average() {
    {
        let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
        let expected = vec![-1, -1, -1, 5, 4, 4, -1, -1, -1];
        let ret = get_averages(nums, 3);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![100000];
        let expected = vec![100000];
        let ret = get_averages(nums, 0);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![8];
        let expected = vec![-1];
        let ret = get_averages(nums, 100000);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![100000];
        let expected = vec![-1];
        let ret = get_averages(nums, 1);
        assert_eq!(ret, expected);
    }
}
