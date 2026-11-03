fn min_moves(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap();
    nums.into_iter().fold(0, |acc, n| acc + max - n)
}

fn main() {
    let ret = min_moves(vec![2, 1, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_moves(vec![2, 1, 3]), 3);
    assert_eq!(min_moves(vec![4, 4, 5]), 2);
}
