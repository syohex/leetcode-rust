use std::collections::HashSet;

fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let s: HashSet<i32> = candy_type.iter().copied().collect();
    std::cmp::min(s.len(), candy_type.len() / 2) as i32
}

fn main() {
    let ret = distribute_candies(vec![1, 1, 2, 2, 3, 3]);
    println!("ret={}", ret);
}

#[test]
fn test_distribute_candies() {
    assert_eq!(distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    assert_eq!(distribute_candies(vec![1, 1, 2, 3]), 2);
    assert_eq!(distribute_candies(vec![6, 6, 6, 6]), 1);
}
