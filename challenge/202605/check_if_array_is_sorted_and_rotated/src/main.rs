fn check(nums: Vec<i32>) -> bool {
    let mut invalids = 0;
    let mut prev = nums[nums.len() - 1];

    for n in nums {
        if prev > n {
            invalids += 1;
        }
        prev = n;
    }

    invalids <= 1
}

fn main() {
    let ret = check(vec![3, 4, 5, 1, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(check(vec![6, 10, 6]));
    assert!(check(vec![7, 9, 1, 1, 1, 3]));
    assert!(check(vec![3, 4, 5, 1, 2]));
    assert!(!check(vec![2, 1, 3, 4]));
    assert!(check(vec![1, 2, 3]));
}
