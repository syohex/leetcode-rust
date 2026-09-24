fn smallest_index(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .enumerate()
        .position(|(i, mut n)| {
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }

            sum == i as i32
        })
        .map(|i| i as i32)
        .unwrap_or(-1)
}

fn main() {
    let ret = smallest_index(vec![1, 3, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(smallest_index(vec![1, 3, 2]), 2);
    assert_eq!(smallest_index(vec![1, 10, 11]), 1);
    assert_eq!(smallest_index(vec![1, 2, 3]), -1);
}
