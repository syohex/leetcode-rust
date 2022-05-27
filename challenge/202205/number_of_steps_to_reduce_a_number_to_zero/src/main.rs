fn number_of_steps(num: i32) -> i32 {
    let mut num = num;
    let mut ret = 0;

    while num != 0 {
        num = if num % 2 == 0 { num / 2 } else { num - 1 };
        ret += 1;
    }

    ret
}

fn main() {
    println!("number_of_steps(123)={}", number_of_steps(123));
}

#[test]
fn test_number_of_steps() {
    assert_eq!(number_of_steps(14), 6);
    assert_eq!(number_of_steps(8), 4);
    assert_eq!(number_of_steps(123), 12);
}
