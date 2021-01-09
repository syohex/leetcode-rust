fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right)
        .into_iter()
        .filter(|n| {
            n.to_string()
                .chars()
                .all(|c| c != '0' && n % c.to_digit(10).unwrap() as i32 == 0)
        })
        .collect()
}

fn main() {
    println!(
        "self_dividing_numbers(1, 22)={:?}",
        self_dividing_numbers(1, 22)
    );
}

#[test]

fn test_self_dividing_numbers() {
    assert_eq!(
        self_dividing_numbers(1, 22),
        vec![1i32, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    )
}
