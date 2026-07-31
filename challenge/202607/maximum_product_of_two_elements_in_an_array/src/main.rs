fn max_product(nums: Vec<i32>) -> i32 {
    let (a, b) = nums.into_iter().fold((i32::MIN, i32::MIN), |(a, b), n| {
        if n > a {
            (n, a)
        } else if n > b {
            (a, n)
        } else {
            (a, b)
        }
    });
    (a - 1) * (b - 1)
}

fn main() {
    let ret = max_product(vec![3, 4, 5, 2]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_product(vec![3, 4, 5, 2]), 12);
    assert_eq!(max_product(vec![1, 5, 4, 5]), 16);
    assert_eq!(max_product(vec![3, 7]), 12);
}
