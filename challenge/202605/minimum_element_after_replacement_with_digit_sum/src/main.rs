fn min_element(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(i32::MAX, |acc, n| {
        let mut n = n;
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        std::cmp::min(acc, sum)
    })
}

fn main() {
    let ret = min_element(vec![999, 19, 199]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_element(vec![10, 12, 13, 14]), 1);
    assert_eq!(min_element(vec![1, 2, 3, 4]), 1);
    assert_eq!(min_element(vec![999, 19, 199]), 10);
}
