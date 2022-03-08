fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .filter(|&&n| n >= 0)
        .zip(nums.iter().filter(|&&n| n < 0))
        .fold(vec![], |mut acc, (&n1, &n2)| {
            acc.push(n1);
            acc.push(n2);
            acc
        })
}

fn main() {
    let nums = vec![3, 1, -2, -5, 2, -4];
    let ret = rearrange_array(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_rearrange_array() {
    {
        let nums = vec![3, 1, -2, -5, 2, -4];
        let expected = vec![3, -2, 1, -5, 2, -4];
        assert_eq!(rearrange_array(nums), expected);
    }
    {
        let nums = vec![-1, 1];
        let expected = vec![1, -1];
        assert_eq!(rearrange_array(nums), expected);
    }
}
