fn find_complement(num: i32) -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut num = num;

    while num > 0 {
        match num % 2 {
            0 => v.push(1),
            1 => v.push(0),
            _ => panic!("never reach here"),
        }
        num /= 2;
    }

    v.iter().rev().fold(0, |acc, n| acc * 2 + *n)
}

fn main() {
    println!("ret={}", find_complement(5));
}

#[test]
fn test_find_complement() {
    assert_eq!(find_complement(5), 2);
    assert_eq!(find_complement(1), 0);
}
