fn min_operations(n: i32) -> i32 {
    let half = n / 2;
    (0..half).into_iter().fold(0, |acc, i| {
        let num = 2 * i + 1;
        acc + (n - num)
    })
}

fn main() {
    println!("ret={}", min_operations(3));
}

#[test]
fn test_min_operations() {
    assert_eq!(min_operations(1), 0);
    assert_eq!(min_operations(2), 1);
    assert_eq!(min_operations(3), 2);
    assert_eq!(min_operations(6), 9);
}
