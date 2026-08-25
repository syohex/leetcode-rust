fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashSet;

    let s: HashSet<_> = nums.into_iter().collect();
    let mut n = k;
    while s.contains(&n) {
        n += k;
    }

    n
}

fn main() {
    let ret = missing_multiple(vec![1, 4, 7, 10, 14], 5);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(missing_multiple(vec![8, 2, 3, 4, 6], 2), 10);
    assert_eq!(missing_multiple(vec![1, 4, 7, 10, 15], 5), 5);
}
