fn sum_counts(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut ret = 0;
    let len = nums.len();
    for i in 0..len {
        let mut s = HashSet::new();
        for j in i..len {
            s.insert(nums[j]);
            ret += s.len() * s.len();
        }
    }

    ret as i32
}

fn main() {
    let ret = sum_counts(vec![1, 2, 1]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sum_counts(vec![1, 2, 1]), 15);
    assert_eq!(sum_counts(vec![1, 1]), 3);
}
