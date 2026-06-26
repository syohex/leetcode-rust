fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.append(&mut nums.clone().into_iter().rev().collect());
    nums
}

fn main() {
    let ret = concat_with_reverse(vec![1, 2, 3]);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    assert_eq!(concat_with_reverse(vec![1, 2, 3]), vec![1, 2, 3, 3, 2, 1]);
    assert_eq!(concat_with_reverse(vec![1]), vec![1, 1]);
}
