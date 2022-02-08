fn add_digits(num: i32) -> i32 {
    let mut num = num;

    loop {
        let mut sum = 0;
        let mut n = num;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }

        if sum < 10 {
            return sum;
        }

        num = sum;
    }
}

fn main() {
    println!("ret={}", add_digits(38));
}

#[test]
fn test_add_digits() {
    assert_eq!(add_digits(38), 2);
    assert_eq!(add_digits(0), 0);
}
