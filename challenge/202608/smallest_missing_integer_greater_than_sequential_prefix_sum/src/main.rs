fn missing_integer(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut prefix_sum = nums[0];
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] + 1 {
            break;
        }

        prefix_sum += nums[i];
    }

    let s: HashSet<_> = nums.into_iter().collect();
    loop {
        if !s.contains(&prefix_sum) {
            return prefix_sum;
        }
        prefix_sum += 1;
    }
}

fn main() {
    let ret = missing_integer(vec![1, 2, 3, 2, 5]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(
        missing_integer(vec![14, 9, 6, 9, 7, 9, 10, 4, 9, 9, 4, 4]),
        15
    );
    assert_eq!(missing_integer(vec![1, 2, 3, 2, 5]), 6);
    assert_eq!(missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
    assert_eq!(
        missing_integer(vec![29, 30, 31, 32, 33, 34, 35, 36, 37]),
        297
    );
    assert_eq!(missing_integer(vec![4, 5, 6, 7, 8, 8, 9, 4, 3, 2, 7]), 30);
    assert_eq!(missing_integer(vec![37, 1, 2, 9, 5, 8, 5, 2, 9, 4]), 38);
    assert_eq!(missing_integer(vec![37, 1, 2, 9, 5, 8, 5, 2, 9, 4]), 38);
}
