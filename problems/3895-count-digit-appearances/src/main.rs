fn count_digit_occurences(nums: Vec<i32>, digit: i32) -> i32 {
    nums.into_iter().fold(0, |acc, n| {
        let mut n = n;
        let mut count = 0;
        while n > 0 {
            if n % 10 == digit {
                count += 1;
            }
            n /= 10;
        }

        acc + count
    })
}
fn main() {
    let ret = count_digit_occurences(vec![12, 54, 32, 22], 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_digit_occurences(vec![12, 54, 32, 22], 2), 4);
    assert_eq!(count_digit_occurences(vec![1, 34, 7], 9), 0);
}
