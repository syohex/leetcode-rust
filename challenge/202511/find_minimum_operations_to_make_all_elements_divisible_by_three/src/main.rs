fn minimum_operations(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, n| match n % 3 {
        0 => acc,
        1 | 2 => acc + 1,
        _ => unreachable!("never reach here"),
    })
}

fn main() {
    let ret = minimum_operations(vec![1, 2, 3, 4]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_operations(vec![1, 2, 3, 4]), 3);
    assert_eq!(minimum_operations(vec![3, 6, 9]), 0)
}
