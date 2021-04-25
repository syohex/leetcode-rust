fn sum_base(n: i32, k: i32) -> i32 {
    let mut ret = 0;
    let mut n = n;
    while n > 0 {
        ret += n % k;
        n /= k;
    }

    ret
}

fn main() {
    let ret = sum_base(34, 6);
    println!("ret={}", ret);
}

#[test]
fn test_sum_base() {
    assert_eq!(sum_base(34, 6), 9);
    assert_eq!(sum_base(10, 10), 1);
}
