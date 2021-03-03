fn missing_number(nums: Vec<i32>) -> i32 {
    nums.iter()
        .enumerate()
        .fold(nums.len() as i32, |acc, (i, &n)| acc ^ (i as i32) ^ n)
}

fn main() {
    let ret = missing_number(vec![3, 0, 1]);
    println!("ret={}", ret);
}

#[test]
fn test_missing_number() {
    assert_eq!(missing_number(vec![3, 0, 1]), 2);
    assert_eq!(missing_number(vec![0, 1]), 2);
    assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    assert_eq!(missing_number(vec![0]), 1);
    assert_eq!(missing_number(vec![]), 0);
}
