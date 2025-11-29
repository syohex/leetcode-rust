fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter().sum::<i32>() % k
}

fn main() {
    let ret = min_operations(vec![3, 9, 7], 5);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_operations(vec![3, 9, 7], 5), 4);
    assert_eq!(min_operations(vec![4, 1, 3], 4), 0);
    assert_eq!(min_operations(vec![3, 2], 6), 5);
}
