use std::collections::HashMap;

fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut h: HashMap<i32, i32> = HashMap::new();
    for &n in &nums {
        *h.entry(n).or_insert(0) += 1;
    }

    h.iter()
        .fold(0, |acc, (&k, &v)| if v == 1 { acc + k } else { acc })
}

fn main() {
    println!(
        "sum_of_unique([1,2,3,2]={}",
        sum_of_unique(vec![1, 2, 3, 2])
    );
}

#[test]
fn test_sum_of_unique() {
    assert_eq!(sum_of_unique(vec![1, 2, 3, 2]), 4);
    assert_eq!(sum_of_unique(vec![1, 1, 1, 1, 1]), 0);
    assert_eq!(sum_of_unique(vec![1, 2, 3, 4, 5]), 15);
}
