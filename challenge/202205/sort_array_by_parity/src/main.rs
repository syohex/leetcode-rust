fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;

    nums.sort_unstable_by(|&a, &b| match (a % 2, b % 2) {
        (0, 0) => a.cmp(&b),
        (0, 1) => std::cmp::Ordering::Less,
        (1, 0) => std::cmp::Ordering::Greater,
        (_, _) => a.cmp(&b),
    });

    nums
}

fn main() {
    let nums = vec![3, 1, 2, 4];
    let ret = sort_array_by_parity(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_sort_array_by_parity() {
    {
        let nums = vec![3, 1, 2, 4];
        let expected = vec![2, 4, 1, 3];
        let ret = sort_array_by_parity(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![0];
        let expected = vec![0];
        let ret = sort_array_by_parity(nums);
        assert_eq!(ret, expected);
    }
}
