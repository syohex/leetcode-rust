fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter()
        .fold((vec![], 0), |(mut acc, sum), n| {
            acc.push(sum + n);
            (acc, sum + n)
        })
        .0
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let ret = running_sum(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_running_sum() {
    {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![1, 3, 6, 10];
        let ret = running_sum(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 1, 1, 1, 1];
        let expected = vec![1, 2, 3, 4, 5];
        let ret = running_sum(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![3, 1, 2, 10, 1];
        let expected = vec![3, 4, 6, 16, 17];
        let ret = running_sum(nums);
        assert_eq!(ret, expected);
    }
}
