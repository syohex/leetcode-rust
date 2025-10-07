fn alternating_sum(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .enumerate()
        .fold(0, |acc, (i, n)| if i % 2 == 0 { acc + n } else { acc - n })
}

fn main() {
    let ret = alternating_sum(vec![1, 3, 5, 7]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(alternating_sum(vec![1, 3, 5, 7]), -4);
    assert_eq!(alternating_sum(vec![100]), 100);
}
