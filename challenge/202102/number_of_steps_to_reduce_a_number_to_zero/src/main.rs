fn number_of_steps(num: i32) -> i32 {
    let mut n = num;
    let mut count = 0;
    while n != 0 {
        n = if n % 2 == 0 { n / 2 } else { n - 1 };
        count += 1;
    }

    count
}

fn main() {
    println!("number_of_steps(14)={}", number_of_steps(14));
}

#[test]
fn test_number_of_steps() {
    assert_eq!(number_of_steps(14), 6);
    assert_eq!(number_of_steps(8), 4);
}
