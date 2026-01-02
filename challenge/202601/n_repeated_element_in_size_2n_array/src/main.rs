fn repeated_n_times(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut visited = HashSet::new();
    for n in nums {
        if visited.contains(&n) {
            return n;
        }
        visited.insert(n);
    }
    unreachable!("never reach here");
}

fn main() {
    let ret = repeated_n_times(vec![2, 1, 2, 5, 3, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(repeated_n_times(vec![1, 2, 3, 3]), 3);
    assert_eq!(repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
    assert_eq!(repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    assert_eq!(repeated_n_times(vec![9, 5, 3, 3]), 3);
}
