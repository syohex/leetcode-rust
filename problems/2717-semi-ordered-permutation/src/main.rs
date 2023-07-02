fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let p1 = nums.iter().position(|s| *s == 1).unwrap();
    let p2 = nums.iter().position(|s| *s == len as i32).unwrap();

    let ret = (p1 + len - 1 - p2) as i32;
    if p1 < p2 {
        ret
    } else {
        ret - 1
    }
}

fn main() {
    let nums = vec![2, 1, 4, 3];
    let ret = semi_ordered_permutation(nums);
    println!("ret={ret}");
}

#[test]
fn test_semi_ordered_permutation() {
    {
        let nums = vec![2, 1, 4, 3];
        let ret = semi_ordered_permutation(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![2, 4, 1, 3];
        let ret = semi_ordered_permutation(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![1, 3, 4, 2, 5];
        let ret = semi_ordered_permutation(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![1, 2];
        let ret = semi_ordered_permutation(nums);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![2, 1];
        let ret = semi_ordered_permutation(nums);
        assert_eq!(ret, 1);
    }
}
