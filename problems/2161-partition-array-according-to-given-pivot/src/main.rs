fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut ret = vec![];
    let pivot_count = nums.iter().filter(|n| **n == pivot).count();
    nums.iter()
        .filter(|&&n| n < pivot)
        .for_each(|&n| ret.push(n));

    (0..pivot_count).for_each(|_| ret.push(pivot));

    nums.iter()
        .filter(|&&n| n > pivot)
        .for_each(|&n| ret.push(n));
    ret
}

fn main() {
    let nums = vec![9, 12, 5, 10, 14, 3, 10];
    let ret = pivot_array(nums, 10);
    println!("ret={:?}", ret);
}

#[test]
fn test_pivot_array() {
    {
        let nums = vec![9, 12, 5, 10, 14, 3, 10];
        let expected = vec![9, 5, 3, 10, 10, 12, 14];
        let ret = pivot_array(nums, 10);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![-3, 4, 3, 2];
        let expected = vec![-3, 2, 4, 3];
        let ret = pivot_array(nums, 2);
        assert_eq!(ret, expected);
    }
}
