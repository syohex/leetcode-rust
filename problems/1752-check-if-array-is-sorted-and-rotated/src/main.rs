fn check(nums: Vec<i32>) -> bool {
    nums.iter()
        .enumerate()
        .filter(|(i, &n)| {
            let prev = nums[(i + nums.len() - 1) % nums.len()];
            prev > n
        })
        .count()
        <= 1
}

fn main() {
    println!("check([3,4,5,1,2]={}", check(vec![3, 4, 5, 1, 2]));
}

#[test]
fn test_check() {
    assert!(check(vec![3, 4, 5, 1, 2]));
    assert!(!check(vec![2, 1, 3, 4]));
    assert!(check(vec![1, 2, 3]));
    assert!(check(vec![1, 1, 1]));
    assert!(check(vec![2, 1]));
    assert!(check(vec![1]));
    assert!(check(vec![6, 10, 6]));
}
