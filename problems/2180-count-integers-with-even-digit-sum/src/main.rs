fn count_even(num: i32) -> i32 {
    (1..=num)
        .into_iter()
        .filter(|n| {
            let mut n = *n;
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }

            sum % 2 == 0
        })
        .count() as i32
}

fn main() {
    println!("ret={}", count_even(30));
}

#[test]
fn test_count_even() {
    assert_eq!(count_even(4), 2);
    assert_eq!(count_even(30), 14);
}
