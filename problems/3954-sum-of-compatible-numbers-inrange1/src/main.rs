fn sum_of_good_integeras(n: i32, k: i32) -> i32 {
    let start = std::cmp::max(1, n - k);
    let end = n + k;

    let mut ret = 0;
    for i in start..=end {
        if (n - i).abs() <= k && (n & i) == 0 {
            ret += i;
        }
    }

    ret
}

fn main() {
    let ret = sum_of_good_integeras(2, 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sum_of_good_integeras(2, 3), 10);
    assert_eq!(sum_of_good_integeras(5, 1), 0);
}
