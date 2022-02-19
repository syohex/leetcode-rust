fn count_operations(num1: i32, num2: i32) -> i32 {
    let mut num1 = num1;
    let mut num2 = num2;

    let mut count = 0;
    loop {
        if num1 == 0 || num2 == 0 {
            return count;
        }

        if num1 > num2 {
            num1 -= num2;
        } else {
            num2 -= num1;
        }

        count += 1;
    }
}

fn main() {
    println!("ret={}", count_operations(2, 3));
}

#[test]
fn test_count_operations() {
    assert_eq!(count_operations(2, 3), 3);
    assert_eq!(count_operations(10, 10), 1);
    assert_eq!(count_operations(0, 0), 0);
}
