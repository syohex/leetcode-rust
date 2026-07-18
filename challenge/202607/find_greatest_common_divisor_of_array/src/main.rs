fn find_gcd(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for n in nums {
        min = std::cmp::min(min, n);
        max = std::cmp::max(max, n);
    }

    while min != 0 {
        (max, min) = (min, max % min);
    }

    max
}

fn main() {
    let ret = find_gcd(vec![3, 3]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(find_gcd(vec![2, 5, 6, 9, 10]), 2);
    assert_eq!(find_gcd(vec![7, 5, 6, 8, 3]), 1);
    assert_eq!(find_gcd(vec![3, 3]), 3);
}
